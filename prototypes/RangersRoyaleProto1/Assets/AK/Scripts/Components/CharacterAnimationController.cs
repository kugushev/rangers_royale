using System;
using System.Collections.Generic;
using CustomizableCharacters;
using UnityEngine;
using Random = UnityEngine.Random;

namespace AK.Scripts.Components
{
    public class CharacterAnimationController : MonoBehaviour
    {
        private static readonly int AnimShowingWeapon = Animator.StringToHash("Showing Weapon");
        private static readonly int AnimSpeed = Animator.StringToHash("Speed");
        private static readonly int AnimDirection = Animator.StringToHash("Direction");
        private static readonly int AnimHurt = Animator.StringToHash("Hurt");
        private static readonly int AnimDie = Animator.StringToHash("Die");
        private static readonly int AnimResurrected = Animator.StringToHash("Resurrected");

        private static readonly IReadOnlyList<int> AttackTriggers = new[]
        {
            Animator.StringToHash("Attack 1"), Animator.StringToHash("Stab")
        };

        [SerializeField] private CustomizableCharacter customizableCharacter;
        [SerializeField] private Animator animator;

        private Vector2 _previousPosition;

        public AnimationDirection CurrentDirection { get; private set; } = AnimationDirection.Down;

        private void Awake()
        {
            ResetRigs();
            animator.SetBool(AnimShowingWeapon, true);
        }

        public void AnimateAttack(Vector3 myPosition, Vector3 targetPosition)
        {
            HandleDirection(targetPosition - myPosition);


            var currentAttack = Random.Range(0, AttackTriggers.Count);
            animator.SetTrigger(AttackTriggers[currentAttack]);
        }

        public void AnimateStun() => animator.SetTrigger(AnimHurt);

        public void AnimateDeath() => animator.SetTrigger(AnimDie);

        public void AnimateResurrect() => animator.SetTrigger(AnimResurrected);

        public void MovementStop() => animator.SetFloat(AnimSpeed, 0);

        public void MovementHandle(Vector2 currentPosition)
        {
            var delta = currentPosition - _previousPosition;
            HandleDirection(delta);

            var speed = delta.magnitude / Time.deltaTime / 2f;
            animator.SetFloat(AnimSpeed, speed);

            _previousPosition = currentPosition;
        }

        private static AnimationDirection GetDirection(Vector2 delta)
        {
            if (delta.magnitude <= Single.Epsilon)
                return AnimationDirection.Down;

            var vec45 = new Vector2(1, 1).normalized;
            var vec315 = new Vector2(1, -1).normalized;
            var cos45 = Vector2.Dot(delta.normalized, vec45);
            var cos315 = Vector2.Dot(delta.normalized, vec315);
            var direction = (cos45, cos315) switch
            {
                (>= 0, <= 0) => AnimationDirection.Up,
                (>= 0, >= 0) => AnimationDirection.Right,
                (< 0, < 0) => AnimationDirection.Left,
                (< 0, > 0) => AnimationDirection.Down,
                _ => AnimationDirection.Down
            };
            return direction;
        }

        private void HandleDirection(Vector2 delta)
        {
            ResetRigs();

            CurrentDirection = GetDirection(delta);

            var (dirInt, rig, scaleXMul) = CurrentDirection switch
            {
                AnimationDirection.Up => (0, customizableCharacter.UpRig, 0),
                AnimationDirection.Down => (2, customizableCharacter.DownRig, 0),
                AnimationDirection.Left => (1, customizableCharacter.SideRig, -1),
                AnimationDirection.Right => (1, customizableCharacter.SideRig, 1),
                _ => throw new ArgumentOutOfRangeException(nameof(CurrentDirection), CurrentDirection, null)
            };

            rig.SetActive(true);

            if (scaleXMul != 0)
            {
                var scale = rig.transform.localScale;
                scale.x = Mathf.Abs(scale.x) * scaleXMul;
                rig.transform.localScale = scale;
            }

            animator.SetFloat(AnimDirection, dirInt);
        }


        private void ResetRigs()
        {
            customizableCharacter.UpRig.SetActive(false);
            customizableCharacter.SideRig.SetActive(false);
            customizableCharacter.DownRig.SetActive(false);
        }

        public enum AnimationDirection
        {
            Up,
            Down,
            Left,
            Right
        }
    }
}