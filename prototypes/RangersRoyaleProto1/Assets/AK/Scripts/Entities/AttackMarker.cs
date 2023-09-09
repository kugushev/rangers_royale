using System;
using AK.Scripts.Entities.Units;
using UnityEngine;

namespace AK.Scripts.Entities
{
    [RequireComponent(typeof(Collider2D))]
    public class AttackMarker : MonoBehaviour
    {
        public event Action<Unit> Hit; 
        
        private void OnTriggerEnter2D(Collider2D other)
        {
            if (other.TryGetComponent(out Unit unit))
            {
                Hit?.Invoke(unit);
            }
        }
    }
}