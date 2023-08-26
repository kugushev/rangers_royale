using System.Collections;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.Serialization;
using Zenject;

namespace AK.Scripts.Entities.Units
{
    public class Dummy: Unit
    {
        [SerializeField] private GameObject selected;
        [FormerlySerializedAs("_currentHp")] [SerializeField]   private float currentHp;
        [Inject] protected readonly PlayerSquad HeroesSquad;
        private readonly WaitForSeconds _wait = new(0.15f);
      

        public override float CurrentHp
        {
            get => currentHp;
            protected set => currentHp = value;
        }

        protected override float Speed => 4f;
        protected override float AttackRange => 1.5f;
        protected override float AttackCooldown => 1f;
        protected override Offence Offence => new(3, 0.25f, 0.2f);
        protected override float MaxHp => 1000;
        protected override float Evasion => 0.2f;
        protected override bool CanParry => true;
        protected override float Parry => 0.2f;
        protected override bool HasShield => true;
        protected override float ShieldArmor => 3;
        protected override float ShieldCoverage => 0.5f;
        protected override float HardArmor => 1f;
        protected override float HardArmorCoverage => 0.2f;
        protected override float SoftArmor => 0.5f;
        protected override float SoftArmorCoverage => 0.2f;
        
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
    }
}