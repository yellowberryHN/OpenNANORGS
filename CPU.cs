using System;
using System.Collections.Generic;

namespace OpenNANORGS
{
    internal class CPU
    {
        /*
         * Optimization notes:
         * - CPU should use `fixed` statement to access memory
         * - Bot energy must be managed efficiently
         * - Every instruction should have a method defined in the CPU, with those methods calling out to Bot
         *   if changing external state.
         * - Support for data structures should be simple and robust
         */
        
        private Bot _bot;
        
        private const ushort MemorySize = 3600;

        public ushort[] Register { get; private set; } = new ushort[14];
        public ushort[] Memory { get; private set; } = new ushort[MemorySize];

        public ushort StackPointer { get; private set; } = MemorySize;
        public ushort InstructionPointer { get; private set; } = 0;
        
        // for debugging mutations
        private Dictionary<ushort, (ushort, ushort)> _mutations = new();

        public CPU(Bot bot, ushort[]? memory = null)
        {
            _bot = bot;
            if (memory != null) Memory = memory;
        }

        private void ConsumeEnergy(int energyUsed = 1)
        {
            _bot.Energy -= (ushort)energyUsed;
        }
        
        public void Execute()
        {
            ConsumeEnergy();
        }
        
        private ushort AdvanceInstructionPointer(ushort step = 3)
        {
            return SetInstructionPointer((ushort)(InstructionPointer + step));
        }

        private ushort SetInstructionPointer(ushort pos)
        {
            if (pos % 3 != 0)
            {
                pos += (ushort)(3 - pos % 3);
            }
            InstructionPointer = (ushort)(pos % MemorySize);
            return InstructionPointer;
        }
        
        public void Cmd_NOP()
        {
            ConsumeEnergy();
            AdvanceInstructionPointer();
        }
        
        

        public void Mutate()
        {
            var pos = (ushort)_bot.Tank.GetNextRand(MemorySize);
            var mut = (ushort)(Memory[pos] ^ (ushort)_bot.Tank.GetNextRand(ushort.MaxValue));
        
            _mutations.TryAdd(pos, (Memory[pos], mut));
        
            Memory[pos] = mut;
        }
    }
}