using System;
using AK.Scripts.Entities;
using AK.Scripts.Entities.Units;
using TMPro;
using UnityEngine;

namespace AK.Scripts.Services
{
    public class ScoreService: MonoBehaviour
    {
        public TextMeshProUGUI youWin;

        private void Update()
        {
            foreach (var artifact in Item.Artifacts)
            {
                if (artifact.Owner is not Hero)
                    return;
            }

            youWin.enabled = true;
            Time.timeScale = 0;
        }
    }
}