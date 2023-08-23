using System.Collections;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.AI;
using UnityEngine.EventSystems;
using Zenject;

namespace AK.Scripts.Entities.Units
{
    public class Hero : Unit
    {
        [SerializeField] private GameObject selectedMark;
        [Inject] private readonly PlayerSquad _squad;
        [Inject] private readonly Bonfire _bonfire;
        [Inject] private readonly World _world;

        private float _maxHp = 5f;
        private float _damage = 1f;

        protected override void OnAwake()
        {
            _squad.AllHeroes.Add(this);
        }

        protected override void DoUpdate()
        {
            if (_bonfire.InHealRange(this))
                SetHp(CurrentHp + Time.deltaTime);
        }

        protected override float Speed => 5f;
        protected override float AttackRange => 2f;
        protected override float AttackCooldown => 1f;
        protected override Damage Damage => new(_damage);
        protected override float MaxHp => _maxHp;

        public override void OnPointerClick(PointerEventData eventData)
        {
            if (eventData.button == PointerEventData.InputButton.Left)
                _squad.SelectHero(this);
        }

        public void HandleSelect() => selectedMark.SetActive(true);
        public void HandleDeselect() => selectedMark.SetActive(false);

        public void CommandMove(Vector2 position) => SetCommand(new Command(position));
        public void CommandAttack(Unit enemy) => SetCommand(new Command(enemy));

        protected override void OnDeath()
        {
            _squad.DeselectHero(this);
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
            for (int i = 9; i >= 0; i--)
            {
                yield return new WaitForSeconds(1);
                SimpleHealthBar.UpdateBar(i, maxValue);
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
                _maxHp += 5f;
                _damage += 1;
                SetHp(MaxHp);
                Destroy(item.gameObject);
            }
        }
    }
}