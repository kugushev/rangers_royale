using UnityEngine;

namespace AK.Scripts.Entities
{
    public struct UnitState
    {
        public int Tick;
        public Vector3 Position;
        public bool Dead;
        public float Hp;
        public float MaxHp;
        public float Damage;
    }
}