using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace OpenNANORGS
{
    class Bot
    {
        private char botId;

        private Playfield playfield;

        public byte x;
        public byte y;

        private int toxic = 0;

        public ushort energy = 10000;

        // instruction pointer
        private ushort ip = 0;

        // stack pointer
        private ushort sp = 3600;

        private ushort[] reg = new ushort[14];

        private Random rnd;

        private ushort r0 = 0;
        private bool tmpSucc = false;

        [Flags]
        private enum BotFlags
        {
            None,
            Success = 1,
            Less = 1 << 1,
            Equal = 1 << 2,
            Greater = 1 << 3,
        }

        private BotFlags flags;

        public Bot(char id, byte x, byte y, Playfield pf)
        {
            rnd = pf.rnd;
            botId = id;
            this.x = x;
            this.y = y;
            playfield = pf;
        }

        private void UseEnergy(byte energy = 1)
        {
            this.energy -= energy;
        }

        private void oper_GETXY(ref ushort argX, ref ushort argY)
        {
            UseEnergy();
            argX = x;
            argY = y;
        }

        private void oper_NOP()
        {
            UseEnergy();
            return; // NOP does nothing, NO OPERATION
        }

        private void oper_SENSE(ref ushort argType)
        {
            UseEnergy();
            argType = playfield.GetElement(x, y);
        }

        private void oper_MOV(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest = src;
        }

        private void oper_ADD(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest += src; 
        }

        private void oper_SUB(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest -= src;
        }

        private void oper_MULT(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest *= src;
        }

        private void oper_DIV(ref ushort dest, ushort src)
        {
            UseEnergy();
            try
            {
                dest /= src;
            }
            catch (DivideByZeroException)
            {
                return;
            }
        }

        private void oper_MOD(ref ushort dest, ushort src)
        {
            UseEnergy();
            try
            {
                dest %= src;
            }
            catch (DivideByZeroException)
            {
                return;
            }
        }

        private void oper_AND(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest &= src;
        }

        private void oper_OR(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest |= src;
        }

        private void oper_XOR(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest ^= src;
        }

        private void oper_SHL(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest <<= src;
        }

        private void oper_SHR(ref ushort dest, ushort src)
        {
            UseEnergy();
            dest >>= src;
        }

        private void oper_RAND(ref ushort dest, ushort max)
        {
            UseEnergy();
            dest = (ushort)rnd.Next(max);
        }

        private void oper_EAT()
        {
            UseEnergy();
            if (energy > 0xFFFF - 2000)
            {
                flags &= ~BotFlags.Success;
                return;
            }
            var id = playfield.Consume(this);
            if(id < 1)
            {
                flags &= ~BotFlags.Success;
                return;
            }

            energy += 2000;

            if(playfield.IsToxic(id))
            {
                toxic++; // just for debugging
                // mutate or some shit, idk
            }

            flags |= BotFlags.Success;
        }

        private void oper_ENERGY(ref ushort dest)
        {
            UseEnergy();
            dest = energy;
        }

        /*
        travel 0 // north
        travel 1 // south
        travel 2 // east
        travel 3 // west

        Moves the organism one slot in the
        specified direction assuming the
        space is no occupied by another
        organism or outside the sludge tank.
        This instruction costs 10 energy
        points if successful; otherwise it
        costs 1 energy point. When an
        organism moves: 
        North: their y coord lessens by 1
        South: their y coord increases by 1
        West: their x coord lessens by 1
        East: their x coord increases by 1
        
        Flags: If the organism successfully moves, the SUCCESS flag is set. Otherwise the SUCCESS flag is cleared.
        */

        private void oper_TRAVEL(ushort dir)
        {
            if(energy < 10)
            {
                flags &= ~BotFlags.Success;
                UseEnergy();
                return;
            }
            bool fail = false;
            switch (dir)
            {
                case 0:
                    if (this.y != 0 && !playfield.Occupied(this.x, this.y - 1)) this.y--;
                    else fail = true;
                    break;
                case 1:
                    if (this.y != 39 && !playfield.Occupied(this.x, this.y + 1)) this.y++;
                    else fail = true;
                    break;
                case 2:
                    if (this.x != 69 && !playfield.Occupied(this.x + 1, this.y)) this.x++;
                    else fail = true;
                    break;
                case 3:
                    if (this.x != 0 && !playfield.Occupied(this.x - 1, this.y)) this.x--;
                    else fail = true;
                    break;
                default:
                    fail = true;
                    break;
            }
            if(fail)
            {
                flags &= ~BotFlags.Success;
                UseEnergy();
                return;
            }
            else
            {
                flags &= BotFlags.Success;
                UseEnergy(10);
                return;
            }
        }

        public void oper_RELEASE(ushort amt)
        {
            UseEnergy();
            if (energy < amt)
            {
                flags &= ~BotFlags.Success;
                return;
            }
            energy -= amt;
            playfield.Collect(this, amt);
        }
        
        public void oper_CMP(ushort op1, ushort op2)
        {
            UseEnergy();
            if (op1 < op2) flags |= BotFlags.Less;
            if (op1 == op2) flags |= BotFlags.Equal;
            if (op1 > op2) flags |= BotFlags.Greater;
        }

        public void Tick(uint tick)
        {
            if(energy < 1) return;
            // actually run instructions for 1 tick here.

            // instruction testing, will not be here in final version
            switch(tick % 7)
            {
                case 0:
                    oper_RAND(ref r0, 4);
                    break;
                case 1:
                    oper_TRAVEL(r0);
                    break;
                case 2:
                    oper_EAT();
                    break;
                case 3:
                    oper_SENSE(ref r0);
                    break;
                case 4:
                    oper_CMP(r0, 0xFFFF);
                    break;
                case 5:
                    UseEnergy();
                    tmpSucc = flags.HasFlag(BotFlags.Equal);
                    break;
                case 6:
                    if(tmpSucc) oper_RELEASE(10000);
                    break;
            }
            
            
            
        }

        public char Render()
        {
            if (energy < 1) return '.';
            else return botId;
        }
    }
}
