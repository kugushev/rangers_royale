using System;
using System.Collections.Generic;
using AK.Scripts.Components;
using AK.Scripts.Services;
using AK.Scripts.ValueObjects;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.AI;
using UnityEngine.EventSystems;
using Zenject;
using Random = UnityEngine.Random;

namespace AK.Scripts.Entities.Units
{
    [RequireComponent(typeof(NavMeshAgent))]
    [RequireComponent(typeof(Collider2D))]
    public abstract class Unit : MonoBehaviour, IPointerClickHandler
    {
        [Inject] protected readonly PauseService PauseService;

        private readonly List<UnitState> _story = new();

        private Command? _currentCommand;
        protected NavMeshAgent Agent;
        private Vector2? _currentDestination;
        protected CharacterAnimationController AnimationController;
        private float _timeSinceLastAttack;
        protected SimpleHealthBar SimpleHealthBar;
        private Collider2D _collider;
        private float _stunCounter;

        protected void Awake()
        {
            Agent = GetComponent<NavMeshAgent>();
            Agent.speed = Speed;

            AnimationController = GetComponentInChildren<CharacterAnimationController>();

            SimpleHealthBar = GetComponentInChildren<SimpleHealthBar>();
            SetHp(MaxHp);

            _collider = GetComponent<Collider2D>();


            PauseService.PausedChanged += PauseChanged;
            OnAwake();
        }

        // private void OnDestroy()
        // {
        //     PauseService.PausedChanged -= PauseChanged;
        // }

        protected virtual void OnAwake()
        {
        }

        private void PauseChanged()
        {
            Agent.enabled = !PauseService.Paused;
            if (!PauseService.Paused && _currentCommand != null && _currentCommand.Value.IsPosition(out var position))
                SetDestinationToAgent(position);
        }

        protected void Update()
        {
            if (PauseService.Paused)
                return;

            ProcessCommands();
            DoUpdate();

            _story.Add(new UnitState
            {
                Dead = Dead,
                Damage = Offence.Damage,
                Hp = CurrentHp,
                MaxHp = MaxHp,
                Position = transform.position,
                Tick = PauseService.Ticks
            });
        }

        public void RestoreState(int ticks, out bool notBorn)
        {
            if (_story.Count == 0 || _story[0].Tick > ticks)
            {
                notBorn = true;
                return;
            }

            var delta = ticks - _story[0].Tick;
            if (_story.Count < delta)
            {
                Debug.LogError($"Shit, too much ticks: {_story.Count} < {delta}");
                delta = _story.Count - 1;
            }

            if (delta < 0) 
                delta = 0;

            notBorn = false;
            var state = _story[delta];
            Dead = state.Dead;
            CurrentHp = state.Hp;
            transform.position = state.Position;
            OnRestore(state);
        }

        protected virtual void OnRestore(UnitState state)
        {
        }

        public void TrimStory(int ticks, out bool notBorn)
        {
            if (_story.Count == 0 || _story[0].Tick > ticks)
            {
                notBorn = true;
                _story.Clear();

                return;
            }

            var delta = ticks - _story[0].Tick;
            if (_story.Count < delta)
            {
                Debug.LogError($"Shit, too much ticks: {_story.Count} < {delta}");
                delta = _story.Count - 1;
            }

            _story.RemoveRange(delta, _story.Count - delta);
            notBorn = false;
        }

        protected virtual void DoUpdate()
        {
        }

        private void OnTriggerEnter2D(Collider2D other)
        {
            if (other.CompareTag("Item") && other.TryGetComponent(out Item item))
            {
                OnItemFound(item);
            }
        }

        protected virtual void OnItemFound(Item item)
        {
        }

        public virtual float CurrentHp { get; protected set; }
        public Command? CurrentCommand => _currentCommand;
        public bool Dead { get; private set; }

        protected abstract float Speed { get; }

        protected abstract float AttackRange { get; }

        protected abstract float AttackCooldown { get; }

        protected abstract Offence Offence { get; }

        protected abstract float MaxHp { get; }

        protected abstract float Evasion { get; }

        protected abstract bool CanParry { get; }
        protected abstract float Parry { get; }

        protected abstract bool HasShield { get; }
        protected abstract float ShieldArmor { get; }
        protected abstract float ShieldCoverage { get; }

        protected abstract float HardArmor { get; }
        protected abstract float HardArmorCoverage { get; }
        protected abstract float SoftArmor { get; }
        protected abstract float SoftArmorCoverage { get; }


        protected internal virtual void HandleAttacked(Offence offence, Unit source)
        {
            if (Dead)
                AnimationController.AnimateDeath();

            var damage = offence.Damage;

            var attackerPosition = GetAttackerPosition();

            if (attackerPosition is RelativePosition.LeftHand or RelativePosition.RightHand)
            {
                // print("Check evade");

                if (!HitCheck(Evasion))
                    return;
            }


            if (CanParry && !offence.Unpaired)
                if (attackerPosition == RelativePosition.RightHand ||
                    (!HasShield && attackerPosition == RelativePosition.LeftHand))
                {
                    // print("Parry Check");

                    if (!HitCheck(Parry))
                    {
                        var parryOffence = new Offence(0f, Offence.Accuracy, Offence.StunTime, true);
                        source.HandleAttacked(parryOffence, this);
                        return;
                    }
                }

            if (HasShield && attackerPosition == RelativePosition.LeftHand)
            {
                // print("Check shield");

                var hit = !HitCheck(ShieldCoverage);
                if (hit)
                {
                    damage = Mathf.Max(damage - ShieldArmor, 0f);
                    // print($"Shield damage reduction to {damage}");
                }
            }

            if (!HitCheck(HardArmorCoverage))
            {
                damage = Mathf.Max(damage - HardArmor, 0f);
                // print($"Hard armor damage reduction to {damage}");
            }

            if (!HitCheck(SoftArmorCoverage))
            {
                damage -= damage * SoftArmor;
                // print($"Soft armor damage reduction to {damage}");
            }

            // print($"Do Damage {CurrentHp} - {damage}");
            SetHp(CurrentHp - damage);

            if (CurrentHp <= 0)
            {
                AnimationController.AnimateDeath();
                Death();
            }
            else
            {
                _stunCounter = offence.StunTime;
                AnimationController.AnimateStun();
            }


            RelativePosition GetAttackerPosition()
            {
                var (direction, direction270) = AnimationController.CurrentDirection switch
                {
                    CharacterAnimationController.AnimationDirection.Up => (Vector2.up, Vector2.right),
                    CharacterAnimationController.AnimationDirection.Down => (Vector2.down, Vector2.left),
                    CharacterAnimationController.AnimationDirection.Left => (Vector2.left, Vector2.up),
                    CharacterAnimationController.AnimationDirection.Right => (Vector2.right, Vector2.down),
                    _ => throw new ArgumentOutOfRangeException()
                };

                Vector2 toSource = (source.transform.position - transform.position).normalized;

                var cos = Vector2.Dot(direction, toSource);
                var cos270 = Vector2.Dot(direction270, toSource);

                return (cos, cos270) switch
                {
                    (>= 0, >= 0) => RelativePosition.RightHand,
                    (>= 0, < 0) => RelativePosition.LeftHand,
                    _ => RelativePosition.Back
                };
            }

            bool HitCheck(float defenceParameter)
            {
                var roll = Random.Range(0f, 1f + offence.Accuracy);
                var hit = roll > defenceParameter;
                // print($"Hit Check: {hit} = {roll} (0-{1f + offence.Accuracy}) > {defenceParameter}");
                return hit;
            }
        }

        private enum RelativePosition
        {
            LeftHand,
            RightHand,
            Back
        }

        public abstract void OnPointerClick(PointerEventData eventData);

        private void ProcessCommands()
        {
            _stunCounter -= Time.deltaTime;
            if (_stunCounter > 0)
                return;

            _timeSinceLastAttack -= Time.deltaTime;

            if (Dead)
                return;

            var myPosition = transform.position;

            if (_currentCommand != null)
            {
                if (_currentCommand.Value.IsUnit(out var target))
                {
                    if (target.IsDestroyed())
                    {
                        _currentCommand = null;
                    }
                    else
                    {
                        var attacking = HandleAttackCommand(target, myPosition);
                        if (attacking)
                            return;
                    }
                }
                else if (_currentCommand.Value.IsPosition(out var position))
                {
                    if (Vector2.Distance(myPosition, position) < Agent.speed * Time.deltaTime)
                        SetCommand(null);
                }
            }

            AnimationController.MovementHandle(myPosition);
        }

        protected virtual void SetCommand(Command? command)
        {
            if (Dead)
                return;

            _currentCommand = command;
            if (command != null && command.Value.IsPosition(out var position))
                SetDestination(position);
        }

        private void SetDestination(Vector2 position)
        {
            if (_currentDestination != null && Vector2.Distance(_currentDestination.Value, position) < Single.Epsilon)
                return;

            _currentDestination = position;
            if (!Agent.enabled)
                return;

            SetDestinationToAgent(position);
        }

        private void SetDestinationToAgent(Vector2 position)
        {
            Agent.isStopped = false;
            Agent.SetDestination(position);
        }

        private bool HandleAttackCommand(Unit target, Vector3 myPosition)
        {
            if (target.Dead)
            {
                _currentCommand = null;
                return false;
            }

            var targetPosition = target.transform.position;
            if (Vector2.Distance(myPosition, targetPosition) > AttackRange)
            {
                SetDestination(targetPosition);
                return false;
            }

            Agent.isStopped = true;
            AnimationController.MovementStop();

            if (_timeSinceLastAttack <= 0)
            {
                AnimationController.AnimateAttack(myPosition, targetPosition);
                _timeSinceLastAttack = AttackCooldown;

                target.HandleAttacked(Offence, this);
                OnTargetAttacked(target);
            }

            return true;
        }

        protected virtual void OnTargetAttacked(Unit target)
        {
        }


        protected void Resurrect()
        {
            Dead = false;
            Agent.isStopped = false;
            _collider.enabled = true;
            AnimationController.AnimateResurrect();
            SetHp(1);
        }

        private void Death()
        {
            Dead = true;
            Agent.isStopped = true;
            _collider.enabled = false;
            OnDeath();
        }

        protected virtual void OnDeath()
        {
        }

        protected void SetHp(float hp)
        {
            CurrentHp = Mathf.Min(hp, MaxHp);
            SimpleHealthBar.UpdateColor(Color.red);
            SimpleHealthBar.UpdateBar(CurrentHp, MaxHp);
        }
    }
}