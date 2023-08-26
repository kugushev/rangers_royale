namespace AK.Scripts.ValueObjects
{
    public struct Offence
    {
        public readonly float Damage;
        public readonly float Accuracy;
        public readonly float StunTime;
        public readonly bool Unpaired;

        public Offence(float damage, float accuracy, float stunTime, bool unpaired = false)
        {
            Damage = damage;
            Accuracy = accuracy;
            StunTime = stunTime;
            Unpaired = unpaired;
        }
    }
}