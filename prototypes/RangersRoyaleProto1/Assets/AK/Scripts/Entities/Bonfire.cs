using AK.Scripts.Entities.Units;
using UnityEngine;

namespace AK.Scripts.Entities
{
    public class Bonfire: MonoBehaviour
    {
        public bool InHealRange(Unit unit) => Vector3.Distance(unit.transform.position, transform.position) < 5f;
    }
}