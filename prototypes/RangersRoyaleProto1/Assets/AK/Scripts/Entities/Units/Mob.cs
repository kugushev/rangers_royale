using System.Collections;
using System.Collections.Generic;
using System.Linq;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.EventSystems;
using Zenject;

namespace AK.Scripts.Entities.Units
{
    public class Mob : Unit
    {
        [SerializeField] private GameObject selected;
        [Inject] protected readonly PlayerSquad HeroesSquad;
        private readonly WaitForSeconds _wait = new(0.15f);

        protected override float Speed => 4f;
        protected override float AttackRange => 1.5f;
        protected override float AttackCooldown => 1f;
        protected override Offence Offence => new(4, 0f, 0.2f);
        protected override float MaxHp => 30;
        protected override float Evasion => 0.2f;
        protected override bool CanParry => true;
        protected override float Parry => 0.2f;
        protected override bool HasShield => false;
        protected override float ShieldArmor => 0;
        protected override float ShieldCoverage => 0f;
        protected override float HardArmor => 3f;
        protected override float HardArmorCoverage => 0.1f;
        protected override float SoftArmor => 0f;
        protected override float SoftArmorCoverage => 0f;

        public override void OnPointerClick(PointerEventData eventData)
        {
            if (eventData.button == PointerEventData.InputButton.Right)
            {
                HeroesSquad.CommandAttack(this);
                StartCoroutine(BlinkSelected());
            }
        }

        private IEnumerator BlinkSelected()
        {
            selected.SetActive(true);
            yield return _wait;
            selected.SetActive(false);
        }

        protected override void HandleAttacked(Offence offence, Unit source)
        {
            base.HandleAttacked(offence, source);
            if (CurrentCommand is null || CurrentCommand.Value.IsPosition(out _))
                SetCommand(new(source));
        }

        protected override void DoUpdate()
        {
            Vector2 myPosition = transform.position;
            AiAggroCheck(myPosition);
            AiRandomWalk(myPosition);
        }

        protected void AiAggroCheck(Vector2 myPosition)
        {
            const float fleeMul = 1.5f;
            if (CurrentCommand is not null && CurrentCommand.Value.IsUnit(out var unit))
            {
                if (unit.Dead || Vector2.Distance(unit.transform.position, myPosition) >= AttackRange * fleeMul)
                    SetCommand(null);
                else
                    return;
            }

            const float aggroRadius = 5f;
            Unit bestTarget = null;
            float bestDistance = float.MaxValue;
            foreach (var hero in HeroesSquad.AllHeroes.Concat<Unit>(Enemy.Enemies))
            {
                if (hero.Dead)
                    continue;

                var distance = Vector2.Distance(hero.transform.position, myPosition);
                if (distance > aggroRadius)
                    continue;

                if (distance < bestDistance)
                {
                    bestTarget = hero;
                    bestDistance = distance;
                }
            }

            if (bestTarget is not null)
                SetCommand(new(bestTarget));
        }

        private float _randomWalkReset;
        protected void AiRandomWalk(Vector2 myPosition)
        {
            const float randomRange = 5f;
            _randomWalkReset -= Time.deltaTime;
            if (CurrentCommand is null || (CurrentCommand.Value.IsPosition(out _) && _randomWalkReset <= 0))
            {
                var target = myPosition;
                target.x += Random.Range(-randomRange, randomRange);
                target.y += Random.Range(-randomRange, randomRange);
                SetCommand(new(target));
                _randomWalkReset = 5f;
            }
        }
    }
}