using AK.Scripts.Entities;
using AK.Scripts.Entities.Units;
using UnityEngine;

namespace AK.Scripts.ValueObjects
{
    public struct Command
    {
        private readonly Vector2? _targetPosition;
        private readonly Unit _targetUnit;

        public Command(Vector2 position)
        {
            _targetPosition = position;
            _targetUnit = null;
        }

        public Command(Unit unit)
        {
            _targetUnit = unit;
            _targetPosition = null;
        }

        public bool IsPosition(out Vector2 position)
        {
            if (_targetPosition != null)
            {
                position = _targetPosition.Value;
                return true;
            }

            position = default;
            return false;
        }        
        
        public bool IsUnit(out Unit unit)
        {
            if (_targetUnit is not null)
            {
                unit = _targetUnit;
                return true;
            }

            unit = default;
            return false;
        }
    }
}