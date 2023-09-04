using System;
using System.Collections.Generic;

namespace OpenNANORGS
{
    class Bot
    {
        public readonly char botId;

        private Tank tank;
        public CPU cpu;

        public byte x;
        public byte y;

        //private int toxic = 0;

        private const ushort MemorySize = 3600;

        public ushort energy = 10000;

        public ushort[] reg = new ushort[14];

        public ushort[] memory = new ushort[MemorySize];
        
        // stack pointer
        // TODO: support the SP operand type
        public ushort sp { get; private set; } = MemorySize;
        
        // instruction pointer
        public ushort ip { get; private set; } = 0;

        public void AdvanceIP(ushort amount = 3)
        {
            ip = (ushort)((ip + amount) % 3600);
        }

        public Dictionary<ushort, ushort> mutations = new();

        public Bot(char id, byte x, byte y, Tank tank, ushort[]? memory = null)
        {
            cpu = new CPU(this, memory, tank.rnd);
            botId = id;
            this.x = x;
            this.y = y;
            this.tank = tank;
        }
        
        public void Tick(uint tick)
        {
            if (energy < 1) return;
            cpu.RunNext();
        }

        private ushort[] CPU_NextInstruction()
        {
            ushort[] inst;
            if (ip % 3 != 0) throw new Exception($"IP is not multiple of 3. IP = {ip}");
            try
            {
                inst = new ushort[3] { memory[ip], memory[ip + 1], memory[ip + 2] };
            }
            catch (IndexOutOfRangeException) // TODO: this is happening weirdly often, look into this.
            {
                ip = 0;
                inst = new ushort[3] { memory[ip], memory[ip + 1], memory[ip + 2] };
            }

            return inst;

        }

        public virtual char Render()
        {
            return energy < 1 ? '.' : botId;
        }
        
        public virtual void Mutate()
        {
            cpu.MutateMemory();
        }
        
        public ushort Consume()
        {
            return tank.Consume(this);
        }

        public bool Collect(ushort amt)
        {
            return tank.Collect(this, amt);
        }

        public ushort GetElement()
        {
            return tank.GetElement(x, y);
        }

        public bool AttemptTravel(ushort dir)
        {
            if (!tank.ValidMove(this, dir)) return false;

            switch (dir % 4)
            {
                case 0:
                    y--;
                    break;
                case 1:
                    y++;
                    break;
                case 2:
                    x++;
                    break;
                case 3: 
                    x--;
                    break;
            }

            return true;
        }
    }

    internal class Drone : Bot
    {
        private static ushort[] _malicious = new ushort[]
        {
            0x8004, 0x000F, 0x0000,
            0x8004, 0x0018, 0x0000,
            0x8004, 0x002A, 0x0000,
            0x8004, 0x0045, 0x0000,
            0x8006, 0xFFF7, 0x0000,
            0x2020, 0x0DFB, 0x0004,
            0x2020, 0x0DFC, 0x000A,
            0x200F, 0x0DFC, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x401E, 0x0002, 0x0000,
            0x800E, 0x000F, 0x0000,
            0x401A, 0x0002, 0x0000,
            0x6017, 0x0002, 0x2710,
            0x8009, 0x0006, 0x0000,
            0x001F, 0x0000, 0x0000,
            0x0005, 0x0000, 0x0000,
            0x4001, 0x0000, 0x0DFB,
            0x6020, 0x0001, 0x0DF8,
            0x5024, 0x0000, 0x0001,
            0x800E, 0x0012, 0x0000,
            0x7017, 0x0000, 0x1000,
            0x800B, 0x000C, 0x0000,
            0x6020, 0x0001, 0x0E10,
            0x7001, 0x0000, 0x1000,
            0x1023, 0x0DFB, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x2017, 0x0DFC, 0x0000,
            0x800B, 0x000F, 0x0000,
            0x001B, 0x0DFB, 0x0000,
            0x800E, 0x0009, 0x0000,
            0x2010, 0x0DFC, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x8004, 0xFFAF, 0x0000,
            0x8006, 0xFFEB, 0x0000
        };
        
        public override char Render()
        {
            return energy < 1 ? ',' : '@';
        }

        public override void Mutate()
        {
            // drones are immune to mutation from toxic sludge, do nothing
            return;
        }

        public Drone(byte x, byte y, Tank tank) : base('@', x, y, tank, _malicious)
        {
            // load malicious code here
        }
    }
}
