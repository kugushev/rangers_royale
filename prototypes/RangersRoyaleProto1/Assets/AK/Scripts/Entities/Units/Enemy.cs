using System.Collections;
using System.Collections.Generic;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.EventSystems;
using Zenject;

namespace AK.Scripts.Entities.Units
{
    public class Enemy : Mob
    {
        [Inject] private readonly World _world;
        [Inject] private readonly Bonfire _bonfire;

        public static readonly List<Enemy> Enemies = new();

        protected override Offence Offence => new(2, 0.0f, 0.2f);
        protected override float MaxHp => 25;
        protected override float Evasion => 0.0f;
        protected override bool CanParry => false;
        protected override float Parry => 0f;
        protected override bool HasShield => true;
        protected override float ShieldArmor => 5;
        protected override float ShieldCoverage => 0.5f;
        protected override float HardArmor => 3f;
        protected override float HardArmorCoverage => 0.8f;
        protected override float SoftArmor => 0f;
        protected override float SoftArmorCoverage => 0f;
        
        protected override void OnAwake()
        {
            Enemies.Add(this);
            base.OnAwake();
        }

        protected override void OnItemFound(Item item)
        {
            if (!item.isArtifact)
                return;

            item.transform.SetParent(transform);
            item.Owner = this;
        }

        protected override void OnDeath()
        {
            base.OnDeath();
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

        protected override void DoUpdate()
        {
            var myPosition = transform.position;
            foreach (var artifact in Item.Artifacts)
            {
                if (Vector3.Distance(artifact.transform.position, myPosition) < 20)
                {
                    if (artifact.Owner == this)
                        continue;

                    if (artifact.Owner is null)
                        SetCommand(new(artifact.transform.position));
                    else
                        SetCommand(new(artifact.Owner));
                    return;
                }
            }

            if (_bonfire.InHealRange(this))
                SetHp(CurrentHp + Time.deltaTime);
            else if (CurrentHp < MaxHp * 0.2f)
                SetCommand(new(_bonfire.transform.position));

            AiRandomWalk(myPosition);
        }
    }
}