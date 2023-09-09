using System;
using System.Collections.Generic;
using UnityEngine;
using Zenject;

namespace AK.Scripts.Entities
{
    public class Spawner: MonoBehaviour
    {
        [Inject] private readonly DiContainer _container;
        
        public static List<Spawner> Spawners = new List<Spawner>();

        public GameObject prefab;

        private void Awake()
        {
            Spawners.Add(this);
        }


        public void Spawn()
        {
            var parent = transform;
            _container.InstantiatePrefab(prefab, parent.position, Quaternion.identity, parent);
        }
    }
}