using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace OpenNANORGS
{
    class Compiler
    {
        public List<CompilerInstruction> instructions = new(1200);

        public string info;

        public Compiler()
        {
            Load(@"../../../bots/testbot.asm");
        }

        public void Load(string filename)
        {
            var lines = File.ReadLines(filename);
            ushort pointer = 0;
            Dictionary<string, ushort> labels = new();
            foreach (var line in lines)
            {
                var l = line.Trim();

                if (l.StartsWith("info: ")) {
                    info = l[6..]; // trim info tag
                    continue;
                }

                // trim comments, we don't need those
                if (l.Contains("//")) l = l.Remove(l.IndexOf("//"));
                if (l.Contains(";"))  l = l.Remove(l.IndexOf(";"));

                l = Regex.Replace(l, "\\s+", " ");
                if (string.IsNullOrWhiteSpace(l)) continue;

                var chop = l.Split(' ', 4);

                int op = ToOpcode(chop[0]);
                if (op != 0xFFFF)
                {
                    pointer += 3;
                }
                else if (chop[0].EndsWith(":"))
                {
                    labels.Add(chop[0].Remove(chop[0].Length - 1, 1), pointer);
                }

                continue;
            }

            foreach (var line in lines)
            {
                var l = line.Trim();

                if (l.StartsWith("info: "))
                {
                    info = l[6..]; // trim info tag
                    continue;
                }

                // trim comments, we don't need those
                if (l.Contains("//")) l = l.Remove(l.IndexOf("//"));
                if (l.Contains(";")) l = l.Remove(l.IndexOf(";"));

                l = Regex.Replace(l, "\\s+", " ");
                if (string.IsNullOrWhiteSpace(l)) continue;

                var chop = l.Split(' ', 4);

                int op = ToOpcode(chop[0]);
                if (op != 0xFFFF)
                {
                    int opcode = op; // for easier manupulation without casts

                    if (op >= 6 && op <= 14 || op == 4) // all call and jump operations are relative EXCEPT WHEN THEY AREN'T >:(
                    {
                        opcode |= 0x8000;
                    }

                    // holy shit this is terrible.
                    // TODO: exterminate this hellspawn

                    ushort arg1 = 0;
                    ushort arg2 = 0;

                    if (chop.Length > 1 && ushort.TryParse((string)chop[1], out ushort result))
                    {
                        arg1 = result;
                    }
                    else if (chop.Length > 1 && chop[1].StartsWith("0x"))
                    {
                        arg1 = Convert.ToUInt16(chop[1], 16);
                    }
                    else if (chop.Length > 1 && labels.ContainsKey(chop[1]))
                    {
                        arg1 = labels[chop[1]];
                    }

                    if (chop.Length > 2 && ushort.TryParse((string)chop[2], out ushort result2))
                    {
                        arg2 = result2;
                    }
                    else if (chop.Length > 2 && chop[2].StartsWith("0x"))
                    {
                        arg2 = Convert.ToUInt16(chop[2], 16);
                    }
                    else if (chop.Length > 2 && labels.ContainsKey(chop[2]))
                    {
                        arg2 = labels[chop[2]];
                    }

                    instructions.Add(new CompilerInstruction((ushort)opcode, arg1, arg2)); // update this.

                    pointer += 3;
                }
            }

            return;
        }

        private ushort ToOpcode(string op)
        {
            switch (op.ToUpper())
            {
                case "MOV": return 1;
                case "PUSH": return 2;
                case "POP": return 3;
                case "CALL": return 4;
                case "RET": return 5;
                case "JMP": return 6;
                case "JL": return 7;
                case "JLE": return 8;
                case "JG": return 9;
                case "JGE": return 10;
                case "JE": return 11;
                case "JNE": return 12;
                case "JS": return 13;
                case "JNS": return 14;
                case "ADD": return 15;
                case "SUB": return 16;
                case "MULT": return 17;
                case "DIV": return 18;
                case "MOD": return 19;
                case "AND": return 20;
                case "OR": return 21;
                case "XOR": return 22;
                case "CMP": return 23;
                case "TEST": return 24;
                case "GETXY": return 25;
                case "ENERGY": return 26;
                case "TRAVEL": return 27;
                case "SHL": return 28;
                case "SHR": return 29;
                case "SENSE": return 30;
                case "EAT": return 31;
                case "RAND": return 32;
                case "RELEASE": return 33;
                case "CHARGE": return 34;
                case "POKE": return 35;
                case "PEEK": return 36;
                case "CKSUM": return 37;

                default:
                    return 0xFFFF;
            }
        }
    }

    class CompilerInstruction
    {
        ushort opcode;

        ushort operand1;
        ushort operand2;

        public CompilerInstruction(ushort opcode, ushort operand1, ushort operand2)
        {
            this.opcode = opcode;
            this.operand1 = operand1;
            this.operand2 = operand2;
        }
    }
}
