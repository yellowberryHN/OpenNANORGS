﻿using System;
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

        private string botSource = string.Empty;

        private byte numSludge;
        private List<byte> toxicSludge;
        private bool debugSludge = true;

        private int seed;
        public Random rnd;

        private uint tick = 0;
        private uint maxtick;

        private ulong score = 0;

        private List<Bot> bots;

        private bool debugBot = false;
        private char debugBotID;
        private Bot dBI;

        private Compiler cmp;

        public bool quiet = false;

        public Playfield(string[] args)
        {
            this.seed = (int)DateTimeOffset.Now.ToUnixTimeSeconds();
            this.maxtick = 1000000;

            ParseArgs(args);

            /*
            if(dBot != 0) 
            {
                debugBot = true;
                debugBotID = dBot;
            }
            */

            rnd = new Random(this.seed);
            numSludge = (byte)rnd.Next(5, 32);

            toxicSludge = new List<byte>();

            for (int i = 0; i < (byte)(numSludge * 0.2); i++)
            {
                toxicSludge.Add((byte)rnd.Next(1, numSludge));
            }

            Randomize();

            bots = new List<Bot>();

            cmp = new Compiler(botSource);

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

            if(debugBot)
            {
                dBI = bots.Find(x => x.botId == debugBotID);
            }
        }

        private void ParseArgs(string[] args)
        {
            foreach (var item in args)
            {
                if(item.Length > 2 && item.StartsWith('-'))
                {
                    var flag = item.Substring(1, 1);
                    switch (flag)
                    {
                        case "p":
                            botSource = item[3..];
                            break;
                        case "s":
                            if (!int.TryParse(item[3..], out this.seed))
                            {
                                throw new Exception("Invalid seed!");
                            }
                            break;
                        case "q":
                            quiet = true;
                            break;
                        default:
                            break;
                    }
                }
            }
        }

        public uint Tick()
        {
            foreach (var bot in bots)
            {
                bot.Tick(tick);
            }
            tick++;
            return tick;
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

            for (var y = 0; y < 40; y++)
            {
                for (var x = 0; x < 70; x++)
                {
                    var occ = false; // block already drawn.
                    foreach (var bot in bots)
                    {
                        if (bot.x != x || bot.y != y) continue;
                        sb.Append(bot.Render());
                        occ = true;
                    }
                    if (!occ)
                    {
                        switch (elements[y, x])
                        {
                            case 0: // empty tile
                                sb.Append(' ');
                                break;
                            case 65535: // collection point
                                sb.Append('$');
                                break;
                            default:
                                if(toxicSludge.Contains((byte)elements[y, x]))
                                {
                                    sb.Append(debugSludge ? '%' : '*');
                                }
                                else
                                {
                                    sb.Append(debugSludge ? elements[y, x].ToString()[0]: '*');
                                }
                                break;
                        }
                    }
                }
            }

            sb.AppendLine($"\r\nScore: {score:n0}, Ticks: {tick:n0} of {maxtick:n0}, Seed: <{seed}>");

            //sb.AppendLine(string.Format("\r\nX: {0:D2}, Y: {1:D2}, Energy: {2:D5}", testBot.x, testBot.y, testBot.energy));

            if (debugBot)
            {
                sb.AppendLine(string.Format("\r\n({0} {1}) x={2}, y={3}, energy={4}, IP={5}, SP={6}, flags={7}    ", cmp.info.Split(',')[0].ToUpper(), dBI.botId, dBI.x, dBI.y, dBI.energy, dBI.ip, dBI.sp, dBI.FlagRender()));
                sb.AppendLine(string.Format("R00={0,5} R01={1,5} R02={2,5} R03={3,5} R04={4,5} R05={5,5} R06={6,5}     ", dBI.reg[0], dBI.reg[1], dBI.reg[2], dBI.reg[3], dBI.reg[4], dBI.reg[5], dBI.reg[6]));
                sb.AppendLine(string.Format("R07={0,5} R08={1,5} R09={2,5} R10={3,5} R11={4,5} R12={5,5} R13={6,5}     ", dBI.reg[7], dBI.reg[8], dBI.reg[9], dBI.reg[10], dBI.reg[11], dBI.reg[12], dBI.reg[13]));
                sb.AppendLine(string.Format("{0:D4}  <not implemented>                                                 ", dBI.ip));
                sb.Append("(u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]: ");
            }

            /*
            (%s %c) x=%d, y=%d, energy=%d, IP=%d, SP=%d, flags=%s                 
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]: 
            Score: %.0lf, Ticks: %d of %d   (Seed=%u)                             
            */

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

        public bool Finished()
        {
            return tick >= maxtick;
        }
    }
}
