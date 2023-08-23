using System;
using AK.Scripts.Components;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.AI;
using UnityEngine.EventSystems;

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

        public float CurrentHp { get; private set; }
        public Command? CurrentCommand => _currentCommand;
        public bool Dead { get; private set; }

        protected abstract float Speed { get; }

        protected abstract float AttackRange { get; }

        protected abstract float AttackCooldown { get; }

        protected abstract Damage Damage { get; }

        protected abstract float MaxHp { get; }

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

                target.ApplyDamage(Damage, this);
            }

            return true;
        }

        protected virtual void ApplyDamage(Damage damage, Unit source)
        {
            if (Dead)
                _animationController.AnimateDeath();

            SetHp(CurrentHp - damage.Amount);

            if (CurrentHp <= 0)
            {
                _animationController.AnimateDeath();
                Death();
            }
            else
            {
                _timeSinceLastAttack = Mathf.Max(_timeSinceLastAttack, damage.StunTime);
                _animationController.AnimateStun();
            }
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