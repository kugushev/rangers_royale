using System;
using System.Collections;
using AK.Scripts.ValueObjects;
using UnityEngine;
using UnityEngine.InputSystem;

namespace AK.Scripts.Entities.Units
{
    public class HeroLink : Hero
    {
        [SerializeField] private float shift = 1f;
        [SerializeField] private AttackMarker attackMarker;

        private float _attackCountdown;
        private WaitForSeconds _attackWait = new WaitForSeconds(0.5f);

        private void Start()
        {
            attackMarker.Hit += HandleHit;
        }

        protected override void DoUpdate()
        {
            if (Dead)
                return;

            base.DoUpdate();
            _attackCountdown -= Time.deltaTime;

            if (Selected)
                HandleKeyboard();
            else
            {
                var hero = Squad.CurrentHero as HeroLink;
                if (hero is null)
                    return;

                if (CurrentCommand == null || CurrentCommand.Value.IsPosition(out _))
                {
                    if (hero.LastTarget is {Dead: not true})
                    {
                        SetCommand(new Command(hero.LastTarget));
                    }
                    else
                    {
                        SetCommand(new (hero.transform.position));
                    }
                }
            }
        }

        protected override float Accuracy => float.MaxValue;

        public override void HandleDeselect()
        {
            base.HandleDeselect();
            if (LastTarget is {Dead: not true})
                SetCommand(new Command(LastTarget));
        }

        public Unit LastTarget;

        private void HandleKeyboard()
        {
            var trans = transform;

            var pos = trans.position;

            var kb = Keyboard.current;

            bool hasCommand = false;

            if (kb.spaceKey.wasPressedThisFrame)
            {
                DoAttack();
                if (CurrentCommand?.IsPosition(out _) == true)
                    SetCommand(null);
                return;
            }

            if (kb.wKey.isPressed)
            {
                pos.y += shift;
                hasCommand = true;
            }
            else if (kb.sKey.isPressed)
            {
                pos.y -= shift;
                hasCommand = true;
            }

            if (kb.dKey.isPressed)
            {
                pos.x += shift;
                hasCommand = true;
            }
            else if (kb.aKey.isPressed)
            {
                pos.x -= shift;
                hasCommand = true;
            }

            if (!hasCommand)
            {
                if (CurrentCommand?.IsPosition(out _) == true)
                    SetCommand(null);
                return;
            }

            SetCommand(new Command(pos));
            LastTarget = null;
        }

        private void DoAttack()
        {
            if (_attackCountdown > 0)
                return;

            attackMarker.gameObject.SetActive(true);

            AnimationController.AnimateAttack();
            StartCoroutine(DisableAttackMarker());

            _attackCountdown = AttackCooldown;
        }

        private IEnumerator DisableAttackMarker()
        {
            yield return _attackWait;
            attackMarker.gameObject.SetActive(false);
        }

        private void HandleHit(Unit target)
        {
            if (target is Hero)
                return;

            target.HandleAttacked(Offence, this);
            attackMarker.gameObject.SetActive(false);
            LastTarget = target;
        }
    }
}