using System;
using System.Collections.Generic;
using AK.Scripts.Entities.Units;
using UnityEngine;

namespace AK.Scripts.Entities
{
    public class Item : MonoBehaviour
    {
        public bool isArtifact;
        public bool isHeart;
        
        private static readonly List<Item> _artifacts = new();
        public static IReadOnlyList<Item> Artifacts => _artifacts;

        private void Awake()
        {
            if (isArtifact) 
                _artifacts.Add(this);
        }


        public Unit Owner { get; set; }
    }
}