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

        protected override void OnAwake()
        {
            Enemies.Add(this);
            base.OnAwake();
        }

        protected override void OnItemFound(Item item)
        {
            item.transform.SetParent(transform);
            item.Owner = this;
            // item.gameObject.SetActive(false);
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