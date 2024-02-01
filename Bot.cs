using SadConsole;
using SadRogue.Primitives;

namespace OpenNANORGS
{
    internal class Bot
    {
        public Tank Tank { get; private set; }
        public char Name { get; private set; }

        public CPU CPU { get; protected set; }

        public ColoredGlyph Glyph { get; protected set; }

        public byte X => (byte)Position.X;
        public byte Y => (byte)Position.Y;

        public Point Position { get; private set; }

        public ushort Energy { get; internal set; } = 10000;

        public bool Hibernating { get; private set; } = false;

        private static ColoredGlyph _hibernation = new(Color.Red, Color.Transparent, '.');
        private static ColoredGlyph _active;

        public Bot(char name, Point position, Tank tank)
        {
            Name = name;
            Glyph = new ColoredGlyph(Color.Orange, Color.Transparent, Name);
            _active = Glyph;
            Position = position;
            Tank = tank;
            CPU = new CPU(this);
            DrawGameObject(Tank.Map);
        }

        public void Tick()
        {
            if (Energy < 1)
            {
                if(!Hibernating) Hibernate();
                return;
            }

            if(Hibernating) Resume();
            
            // debug stuff
            if (Tank.currentTick % 10 == 0)
            {
                Travel(Tank.GetNextRand(4));
                Consume(Tank);
            }
            
            CPU.Execute();
            // run CPU instruction here
        }

        private void Hibernate()
        {
            Hibernating = true;
            Glyph = _hibernation;
            DrawGameObject(Tank.Map);
        }

        private void Resume()
        {
            Hibernating = false;
            Glyph = _active;
            DrawGameObject(Tank.Map);
        }

        public virtual void Mutate()
        {
            CPU.Mutate();
        }

        public bool Travel(int direction)
        {
            switch (direction % 4)
            {
                case 0: // north
                    return Move(Position + Direction.Up);
                case 1: // south
                    return Move(Position + Direction.Down);
                case 2: // east
                    return Move(Position + Direction.Right);
                case 3: // west
                    return Move(Position + Direction.Left);
            }

            return false;
        }

        /// TODO: this should only return the sludge type, energy should be handled by the CPU
        public ushort Consume(Tank tank)
        {
            var sludge = tank.Consume(this);
            if (sludge != 0) Energy += 2000;
            return sludge;
        }
        
        private bool Move(Point newPosition)
        {
            // Check if other bot is in the way
            if (Tank.IsOccupied(newPosition)) return false;

            // Restore the old cell
            Tank.RestoreMapGlyph(Position);

            Position = newPosition;
            DrawGameObject(Tank.Map);

            return true;
        }
        
        private void DrawGameObject(IScreenSurface screenSurface)
        {
            Glyph.CopyAppearanceTo(screenSurface.Surface[Position]);
            screenSurface.IsDirty = true;
        }
    }
}