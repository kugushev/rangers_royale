using System.Collections;
using AK.Scripts.ValueObjects;
using TMPro;
using UnityEngine;
using UnityEngine.AI;
using UnityEngine.EventSystems;
using Zenject;

namespace AK.Scripts.Entities.Units
{
    public class Hero : Unit
    {
        [SerializeField] private GameObject selectedMark;
        [SerializeField] private bool hasShield;
        [SerializeField] private float hardArmorCoverage = 0.2f;
        [SerializeField] private float softArmorCoverage = 0.2f;
        [SerializeField] private float startLevel = 0;
        [SerializeField] private TextMeshProUGUI lvlUpText;

        [Inject] protected readonly PlayerSquad Squad;
        [Inject] private readonly Bonfire _bonfire;
        [Inject] private readonly World _world;


        private float _maxHp = 20f;
        private float _damage = 1f;

        protected override void OnAwake()
        {
            Squad.AllHeroes.Add(this);
            for (int i = 0; i < startLevel; i++)
                HandleLevelUp();
        }

        protected override void DoUpdate()
        {
            if (Dead)
                return;

            if (_bonfire.InHealRange(this))
                SetHp(CurrentHp + Time.deltaTime);

            if (CurrentCommand == null)
            {
                const float aggroRadius = 20f;
                Unit bestTarget = null;
                float bestDistance = float.MaxValue;
                foreach (var mob in Mob.AllMobs)
                {
                    if (mob is Enemy)
                        continue;

                    if (mob.Dead)
                        continue;

                    var distance = Vector2.Distance(mob.transform.position, transform.position);
                    if (distance > aggroRadius)
                        continue;

                    if (distance < bestDistance)
                    {
                        bestTarget = mob;
                        bestDistance = distance;
                    }
                }

                if (bestTarget is not null)
                    SetCommand(new(bestTarget));
            }
        }

        protected override float Speed => 5f;
        protected override float AttackRange => 2f;
        protected override float AttackCooldown => 1f;
        protected virtual float Accuracy => 0.5f;
        protected override Offence Offence => new(_damage, Accuracy, 0.2f);
        protected override float MaxHp => _maxHp;
        protected override float Evasion => 0.4f;
        protected override bool CanParry => true;
        protected override float Parry => 0.4f;
        protected override bool HasShield => hasShield;
        protected override float ShieldArmor => 100;
        protected override float ShieldCoverage => 100f;
        protected override float HardArmor => 1f;
        protected override float HardArmorCoverage => hardArmorCoverage;
        protected override float SoftArmor => 0.3f;
        protected override float SoftArmorCoverage => softArmorCoverage;

        public override void OnPointerClick(PointerEventData eventData)
        {
            if (eventData.button == PointerEventData.InputButton.Left)
                Squad.SelectHero(this);
        }

        public bool Selected { get; private set; }

        public void HandleSelect()
        {
            Selected = true;
            selectedMark.SetActive(true);
        }

        public virtual void HandleDeselect()
        {
            Selected = false;
            selectedMark.SetActive(false);
        }

        public void CommandMove(Vector2 position) => SetCommand(new Command(position));
        public void CommandAttack(Unit enemy) => SetCommand(new Command(enemy));

        protected override void OnDeath()
        {
            Squad.DeselectHero(this);
            var items = GetComponentsInChildren<Item>();
            foreach (var item in items)
            {
                item.transform.SetParent(_world.transform);
                item.Owner = null;
            }

            StartCoroutine(Respawning());
        }

        private IEnumerator Respawning()
        {
            SimpleHealthBar.UpdateColor(Color.black);
            var maxValue = 10;
            SimpleHealthBar.UpdateBar(maxValue, maxValue);

            int i = 9;

            while (i >= 0)
            {
                yield return new WaitForSeconds(1);
                if (PauseService.Paused)
                    continue;
                SimpleHealthBar.UpdateBar(i, maxValue);
                i--;
            }

            Resurrect();
        }

        protected override void OnItemFound(Item item)
        {
            if (item.isArtifact)
            {
                item.transform.SetParent(transform);
                item.Owner = this;
            }
            else if (item.isHeart)
            {
                HandleLevelUp();
                Destroy(item.gameObject);
            }
        }

        private void HandleLevelUp()
        {
            _maxHp += 5f;
            _damage += 1;
            SetHp(MaxHp);
            StartCoroutine(LvlUpBlink());
        }

        private readonly WaitForSeconds _blinkWait = new(0.5f);

        private IEnumerator LvlUpBlink()
        {
            lvlUpText.gameObject.SetActive(true);
            yield return _blinkWait;
            lvlUpText.gameObject.SetActive(false);
        }

        protected override void OnRestore(UnitState state)
        {
            _damage = state.Damage;
            _maxHp = state.MaxHp;
        }

        protected override void OnTargetAttacked(Unit target)
        {
            if (target.Dead)
                HandleLevelUp();
        }
    }
}