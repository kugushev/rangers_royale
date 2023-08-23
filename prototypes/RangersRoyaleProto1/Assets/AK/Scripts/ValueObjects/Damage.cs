namespace AK.Scripts.ValueObjects
{
    public struct Damage
    {
        public readonly float Amount;
        public readonly float StunTime;

        public Damage(float amount)
        {
            Amount = amount;
            StunTime = 0.2f;
        }
    }
}