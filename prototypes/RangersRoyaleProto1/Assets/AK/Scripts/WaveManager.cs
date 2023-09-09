using System;
using System.Linq;
using AK.Scripts.Entities;
using AK.Scripts.Entities.Units;
using UnityEngine;
using Random = UnityEngine.Random;

namespace AK.Scripts
{
    public class WaveManager : MonoBehaviour
    {
        [SerializeField] private int maxWaves = 10;

        private int _currentWave;

        private void Update()
        {
            if (Mob.AllMobs.Count == 0 || Mob.AllMobs.All(m => m.Dead))
            {
                _currentWave++;

                for (int i = 0; i < _currentWave; i++)
                {
                    var spawner = Spawner.Spawners[Random.Range(0, Spawner.Spawners.Count)];
                    spawner.Spawn();
                }
            }
        }
    }
}