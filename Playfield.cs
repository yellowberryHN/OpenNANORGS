using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace OpenNANORGS
{
    class Playfield
    {
        // hold data about how many sludge types, which are toxic, where collection points are located,
        // where to spawn nanorgs and drones, where all items are on the map.

        private ushort[,] elements = new ushort[40, 70];



        private byte numSludge;
        private List<byte> toxicSludge;
        private bool debugSludge = false;

        private int seed;
        public Random rnd;

        private uint tick = 0;

        private ulong score = 0;

        private List<Bot> bots;

        public Playfield(int seed)
        {
            this.seed = seed;
            rnd = new Random(this.seed);
            numSludge = (byte)rnd.Next(5, 32);

            toxicSludge = new List<byte>();

            for (int i = 0; i < (byte)(numSludge * 0.2); i++)
            {
                toxicSludge.Add((byte)rnd.Next(1, numSludge));
            }

            Randomize();

            bots = new List<Bot>();

            var cmp = new Compiler();

            for (int i = 0; i < 26; i++)
            {
                var bot = new Bot((char)(65 + i), (byte)rnd.Next(70), (byte)rnd.Next(40), this);
                bots.Add(bot);
            }
            for (int i = 0; i < 24; i++) // only 50 bots, poor 'y' and 'z' :(
            {
                var bot = new Bot((char)(97 + i), (byte)rnd.Next(70), (byte)rnd.Next(40), this);
                bots.Add(bot);
            }
        }

        public StringBuilder Tick()
        {
            foreach (var bot in bots)
            {
                bot.Tick(tick);
            }
            var sb = Render();
            tick++;
            return sb;
        }

        // check to see bot can move to specific tile
        public bool Occupied(int x, int y)
        {
            if (x > 69 || y > 39 || x < 0 || y < 0) return true;

            foreach (var bot in bots)
            {
                if (bot.x == x && bot.y == y) return true;
            }

            return false;
        }

        public bool IsToxic(ushort id)
        {
            return toxicSludge.Contains((byte)id);
        }

        public void Randomize()
        {
            elements = new ushort[40, 70];
            for (int i = 0; i < 200; i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), (ushort)rnd.Next(1, numSludge));
                }
            }

            for (int i = 0; i < 10; i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), 0xFFFF);
                }
            }
        }

        public ushort GetElement(byte x, byte y)
        {
            return elements[y, x];
        }

        public ushort Consume(Bot bot)
        {
            var sludge = elements[bot.y, bot.x];
            if (sludge != 0xFFFF && sludge != 0)
            {
                elements[bot.y, bot.x] = 0;
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), sludge);
                }
            }
            else if (sludge == 0xFFFF) sludge = 0; // on a collection point, report back that nothing was consumed
            return sludge;
        }

        public void Collect(Bot bot, ushort amt)
        {
            if(elements[bot.y, bot.x] == 0xFFFF)
            {
                score += amt;
            }
        }

        private bool SetElement(byte x, byte y, ushort id)
        {
            if (elements[y, x] != 0) return false;
            elements[y, x] = id;
            return true;
        }

        private string oldField;

        public StringBuilder Render()
        {
            var sb = new StringBuilder();

            for (int y = 0; y < 40; y++)
            {
                string tmp = string.Empty;
                for (int x = 0; x < 70; x++)
                {
                    bool occ = false; // block already drawn.
                    foreach (var bot in bots)
                    {
                        if(bot.x == x && bot.y == y)
                        {
                            tmp += bot.Render();
                            occ = true;
                        }
                    }
                    if (!occ)
                    {
                        switch (elements[y, x])
                        {
                            case 0: // empty tile
                                tmp += " ";
                                break;
                            case 65535: // collection point
                                tmp += "$";
                                break;
                            default:
                                if(toxicSludge.Contains((byte)elements[y, x]))
                                {
                                    tmp += debugSludge ? "%" : "*";
                                }
                                else
                                {
                                    tmp += debugSludge ? elements[y, x].ToString() : "*";
                                }
                                
                                break;
                        }
                    }
                }
                sb.AppendLine(tmp);
            }

            sb.AppendLine(string.Format("\r\nScore: {0}, Ticks: {1} of 1,000,000, Seed: <{2}>", score, tick, seed));

            //sb.AppendLine(string.Format("\r\nX: {0:D2}, Y: {1:D2}, Energy: {2:D5}", testBot.x, testBot.y, testBot.energy));

            byte li = 0;
            foreach (var bot in bots)
            {
                if (bot.energy > 1) li++;
            }
            sb.AppendLine(string.Format("\r\nLive organisms: {0}", li));

            /*
            Score: 0, Ticks: 0 of 1000000   (Seed=1641076912)

            (AJOSA A) x=45, y=25, energy=10000, IP=0, SP=3600, flags=
            R00=    0 R01=    0 R02=    0 R03=    0 R04=    0 R05=    0 R06=    0
            R07=    0 R08=    0 R09=    0 R10=    0 R11=    0 R12=    0 R13=    0
            0000  travel 0
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]:
            */


            return sb;
        }
    }
}
