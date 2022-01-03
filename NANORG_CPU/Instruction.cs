using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace NANORG_CPU
{
    public class Instruction
    {
        private ushort[] raw = new ushort[3] { 0, 0, 0 };

        public Instruction(CPUOpCode opcode, Operand op1, Operand op2)
        {
            ushort tmp;
            tmp = (ushort)((ushort)opcode | (op1.GetBytesFromMode(op2) << 12));

            raw[0] = tmp;

            tmp = 0;
            switch (op1.type)
            {
                case CPUOperType.Direct:
                case CPUOperType.Immediate:
                case CPUOperType.Register:
                    tmp = op1.value;
                    break;
                case CPUOperType.RegisterIndexed: // TODO: add support for subtraction
                    tmp = (ushort)((ushort)op1.index | ((ushort)op1.value << 12));
                    break;
            }

            raw[1] = tmp;

            tmp = 0;
            switch (op2.type)
            {
                case CPUOperType.Direct:
                case CPUOperType.Immediate:
                case CPUOperType.Register:
                    tmp = op2.value;
                    break;
                case CPUOperType.RegisterIndexed:
                    tmp = (ushort)((ushort)op2.index | ((ushort)op2.value << 12));
                    break;
            }

            raw[2] = tmp;
        }

        public string ToAssembly()
        {
            string tmp;

            tmp = ((CPUOpCode)(this.raw[0] & 0xFF)).ToString();

            var modes = Operand.GetModesFromBytes(this.raw[0]);

            switch (modes[0])
            {
                case CPUOperType.Register:
                    tmp += " R" + this.raw[1];
                    break;
                case CPUOperType.Direct:
                    tmp += " [" + this.raw[1] + "]";
                    break;
                case CPUOperType.Immediate:
                    tmp += " " + this.raw[1];
                    break;
                default:
                    throw new NotImplementedException();
            }

            switch (modes[1])
            {
                case CPUOperType.Register:
                    tmp += ", R" + this.raw[2];
                    break;
                case CPUOperType.Direct:
                    tmp += ", [" + this.raw[2] + "]";
                    break;
                case CPUOperType.Immediate:
                    tmp += ", " + this.raw[2];
                    break;
                default:
                    throw new NotImplementedException();
            }

            return tmp;
            //throw new NotImplementedException();
        }

        public override string ToString()
        {
            return string.Format("{0} {1} {2}", raw.Select(x => x.ToString("X4")).ToArray()); 
        }
    }

    public class Operand
    {
        public readonly CPUOperType type;
        public readonly ushort value;
        public readonly ushort index;
        public readonly bool sub;

        public Operand()
        {
            this.type = CPUOperType.Direct;
            this.value = 0;
            this.index = 0;
            this.sub = false;
        }

        public Operand(CPUOperType type, ushort value, ushort index = 0, bool sub = true)
        {
            this.type = type;
            this.value = value;
            this.index = index;
            this.sub = sub;
        }

        public ushort GetBytesFromMode(Operand op2)
        {
            ushort value = 0;
            value = (ushort)((ushort)this.type << 2); // what the fuck
            value |= (ushort)op2.type;

            return value;
        }

        public static CPUOperType[] GetModesFromBytes(ushort bytes)
        {
            var modes = new CPUOperType[2];

            modes[0] = (CPUOperType)((bytes & 0xC000) >> 14);
            modes[1] = (CPUOperType)((bytes & 0x3000) >> 12);

            return modes;
        }
    }

    [Flags]
    public enum CPUOperType
    {
        Direct = 0,         // 0 0
        Register = 1,       // 0 1
        Immediate = 2,      // 1 0
        RegisterIndexed = 3 // 1 1
    }

    public enum CPUOpCode
    {
        NOP = 0,
        MOV = 1,
        PUSH = 2,
        POP = 3,
        CALL = 4,
        RET = 5,
        JMP = 6,
        JL = 7,
        JLE = 8,
        JG = 9,
        JGE = 10,
        JE = 11,
        JNE = 12,
        JS = 13,
        JNS = 14,
        ADD = 15,
        SUB = 16,
        MULT = 17,
        DIV = 18,
        MOD = 19,
        AND = 20,
        OR = 21,
        XOR = 22,
        CMP = 23,
        TEST = 24,
        GETXY = 25,
        ENERGY = 26,
        TRAVEL = 27,
        SHL = 28,
        SHR = 29,
        SENSE = 30,
        EAT = 31,
        RAND = 32,
        RELEASE = 33,
        CHARGE = 34,
        POKE = 35,
        PEEK = 36,
        CKSUM = 37
    }
}
