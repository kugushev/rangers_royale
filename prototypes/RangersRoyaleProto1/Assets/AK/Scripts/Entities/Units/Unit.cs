using System;
using AK.Scripts.Components;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.AI;
using UnityEngine.EventSystems;
using Random = UnityEngine.Random;

namespace AK.Scripts.Entities.Units
{
    [RequireComponent(typeof(NavMeshAgent))]
    [RequireComponent(typeof(Collider2D))]
    public abstract class Unit : MonoBehaviour, IPointerClickHandler
    {
        private Command? _currentCommand;
        private NavMeshAgent _agent;
        private Vector2? _currentDestination;
        private CharacterAnimationController _animationController;
        private float _timeSinceLastAttack;
        protected SimpleHealthBar SimpleHealthBar;
        private Collider2D _collider;

        protected void Awake()
        {
            _agent = GetComponent<NavMeshAgent>();
            _agent.speed = Speed;

            _animationController = GetComponentInChildren<CharacterAnimationController>();

            SimpleHealthBar = GetComponentInChildren<SimpleHealthBar>();
            SetHp(MaxHp);

            _collider = GetComponent<Collider2D>();

            OnAwake();
        }

        protected virtual void OnAwake()
        {
        }

        protected void Update()
        {
            ProcessCommands();
            DoUpdate();
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


        protected virtual void HandleAttacked(Offence offence, Unit source)
        {
            if (Dead)
                _animationController.AnimateDeath();

            var damage = offence.Damage;

            var attackerPosition = GetAttackerPosition();

            if (attackerPosition is RelativePosition.LeftHand or RelativePosition.RightHand)
            {
                print("Check evade");

                if (!HitCheck(Evasion))
                    return;
            }


            if (CanParry && !offence.Unpaired)
                if (attackerPosition == RelativePosition.RightHand ||
                    (!HasShield && attackerPosition == RelativePosition.LeftHand))
                {
                    print("Parry Check");

                    if (!HitCheck(Parry))
                    {
                        var parryOffence = new Offence(0f, Offence.Accuracy, Offence.StunTime, true);
                        source.HandleAttacked(parryOffence, this);
                        return;
                    }
                }

            if (HasShield && attackerPosition == RelativePosition.LeftHand)
            {
                print("Check shield");

                var hit = !HitCheck(ShieldCoverage);
                if (hit)
                {
                    damage = Mathf.Max(damage - ShieldArmor, 0f);
                    print($"Shield damage reduction to {damage}");
                }
            }

            if (!HitCheck(HardArmorCoverage))
            {
                damage = Mathf.Max(damage - HardArmor, 0f);
                print($"Hard armor damage reduction to {damage}");
            }

            if (!HitCheck(SoftArmorCoverage))
            {
                damage -= damage * SoftArmor;
                print($"Soft armor damage reduction to {damage}");
            }

            print($"Do Damage {CurrentHp} - {damage}");
            SetHp(CurrentHp - damage);

            if (CurrentHp <= 0)
            {
                _animationController.AnimateDeath();
                Death();
            }
            else
            {
                _timeSinceLastAttack = Mathf.Max(_timeSinceLastAttack, offence.StunTime);
                _animationController.AnimateStun();
            }


            RelativePosition GetAttackerPosition()
            {
                var (direction, direction270) = _animationController.CurrentDirection switch
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
                print($"Hit Check: {hit} = {roll} (0-{1f + offence.Accuracy}) > {defenceParameter}");
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
            _timeSinceLastAttack -= Time.deltaTime;

            if (Dead)
                return;

            var myPosition = transform.position;

            if (_currentCommand != null)
            {
                if (_currentCommand.Value.IsUnit(out var target))
                {
                    var attacking = HandleAttackCommand(target, myPosition);
                    if (attacking)
                        return;
                }
                else if (_currentCommand.Value.IsPosition(out var position))
                {
                    if (Vector2.Distance(myPosition, position) < _agent.speed * Time.deltaTime)
                        SetCommand(null);
                }
            }

            _animationController.MovementHandle(myPosition);
        }

        protected void SetCommand(Command? command)
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
            _agent.isStopped = false;
            _agent.SetDestination(position);
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

            _agent.isStopped = true;
            _animationController.MovementStop();

            if (_timeSinceLastAttack <= 0)
            {
                _animationController.AnimateAttack(myPosition, targetPosition);
                _timeSinceLastAttack = AttackCooldown;

                target.HandleAttacked(Offence, this);
            }

            return true;
        }


        protected void Resurrect()
        {
            Dead = false;
            _agent.isStopped = false;
            _collider.enabled = true;
            _animationController.AnimateResurrect();
            SetHp(1);
        }

        private void Death()
        {
            Dead = true;
            _agent.isStopped = true;
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