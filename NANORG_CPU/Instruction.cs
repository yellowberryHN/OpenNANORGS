using System;
using System.Linq;

namespace OpenNANORGS.CPU
{
    public class Instruction
    {
        public ushort[] bytecode = new ushort[3] { 0, 0, 0 };
        private readonly ushort ip;

        public Instruction(CPUOpCode opcode, Operand op1, Operand op2, ushort ip = 0)
        {
            this.ip = ip;
            
            op1 ??= new Operand();
            op2 ??= new Operand();
            
            var tmp = (ushort)((ushort)opcode | (op1.GetBytesFromMode(op2) << 12)); 

            bytecode[0] = tmp;

            tmp = 0;
            switch (op1.type)
            {
                case CPUOperType.Immediate:
                    if (opcode is >= CPUOpCode.JMP and <= CPUOpCode.JNS or CPUOpCode.CALL)
                    {
                        tmp = (ushort)(op1.value - ip);
                    }
                    else tmp = op1.value;
                    break;

                case CPUOperType.Direct:
                case CPUOperType.Register:
                    tmp = op1.value;
                    break;
                case CPUOperType.RegisterIndexed:
                    if (op1.sub && op1.offset != 0) bytecode[0] |= 2 << 10;
                    ushort idx = op1.sub && op1.offset != 0 ? (ushort)(0x1000 - op1.offset) : op1.offset;
                    tmp = (ushort)(idx | (op1.value << 12));
                    break;
                default:
                    throw new Exception("Invalid operand type");
            }

            bytecode[1] = tmp;

            tmp = 0;
            switch (op2.type)
            {
                case CPUOperType.Direct:
                case CPUOperType.Immediate:
                case CPUOperType.Register:
                    tmp = op2.value;
                    break;
                case CPUOperType.RegisterIndexed:
                    if (op2.sub && op2.offset != 0) bytecode[0] |= 1 << 10;
                    ushort idx = op2.sub && op2.offset != 0 ? (ushort)(0x1000 - op2.offset) : op2.offset;
                    tmp = (ushort)(idx | (op2.value << 12));
                    break;
                default:
                    throw new Exception("Invalid operand type");
            }

            bytecode[2] = tmp;
        }

        public string ToAssembly()
        {
            var opcode = (CPUOpCode)(this.bytecode[0] & 0xFF);
            var operands = (int)Enum.Parse<CPUOpCodeOperands>(opcode.ToString());
            var buffer = (opcode).ToString().ToLower();

            var modes = Operand.GetModesFromBytes(this.bytecode[0]);

            if (operands >= 1)
            {
                switch (modes[0])
                {
                    case CPUOperType.Register:
                        buffer += " r" + this.bytecode[1];
                        break;
                    case CPUOperType.Direct:
                        buffer += " [" + this.bytecode[1] + "]";
                        break;
                    case CPUOperType.Immediate:
                        if (opcode is >= CPUOpCode.JMP and <= CPUOpCode.JNS or CPUOpCode.CALL)
                            buffer += " " + (ushort)(this.bytecode[1] + this.ip);
                        else
                            buffer += " " + this.bytecode[1];
                        break;
                    case CPUOperType.RegisterIndexed:
                        var register = (this.bytecode[1] & 0xf000) >> 12;
                        var offset = this.bytecode[1] & 0xfff;
                        var sub = (this.bytecode[0] & 0x800) == 0x800;
                    
                        if (offset == 0) buffer += " [r" + register + "]";
                        else
                        {
                            if (sub) /* subtract */ buffer += " [r" + register + "-" + (0x1000 - offset) + "]";
                            else buffer += " [r" + register + "+" + offset + "]";
                        }
                        break;
                    default:
                        throw new NotImplementedException();
                }
            }

            if (operands == 2)
            {
                switch (modes[1])
                {
                    case CPUOperType.Register:
                        buffer += ", r" + this.bytecode[2];
                        break;
                    case CPUOperType.Direct:
                        buffer += ", [" + this.bytecode[2] + "]";
                        break;
                    case CPUOperType.Immediate:
                        buffer += ", " + this.bytecode[2];
                        break;
                    case CPUOperType.RegisterIndexed:
                        var register = (this.bytecode[2] & 0xf000) >> 12;
                        var offset = this.bytecode[2] & 0xfff;
                        var sub = (this.bytecode[0] & 0x400) == 0x400;
                    
                        if (offset == 0) buffer += ", [r" + register + "]";
                        else
                        {
                            if (sub) /* subtract */ buffer += ", [r" + register + "-" + (0x1000 - offset) + "]";
                            else buffer += ", [r" + register + "+" + offset + "]";
                        }
                        break;
                    default:
                        throw new NotImplementedException();
                }
            }
            
            return buffer;
        }

        public override string ToString() => string.Format("{0} ({1})", ToAssembly().PadRight(30), string.Format("{0} {1} {2}", bytecode.Select(x => x.ToString("X4")).ToArray()));
    }

    public class Operand
    {
        public readonly CPUOperType type;
        public readonly ushort value;
        public readonly ushort offset;
        public readonly bool sub;

        public Operand()
        {
            this.type = CPUOperType.Direct;
            this.value = 0;
            this.offset = 0;
            this.sub = false;
        }

        public Operand(CPUOperType type, ushort value, ushort offset = 0, bool sub = false)
        {
            this.type = type;
            this.value = value;
            this.offset = offset;
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

    public enum CPUOpCodeOperands
    {
        NOP = 0,
        MOV = 2,
        PUSH = 1,
        POP = 1,
        CALL = 1,
        RET = 0,
        JMP = 1,
        JL = 1,
        JLE = 1,
        JG = 1,
        JGE = 1,
        JE = 1,
        JNE = 1,
        JS = 1,
        JNS = 1,
        ADD = 2,
        SUB = 2,
        MULT = 2,
        DIV = 2,
        MOD = 2,
        AND = 2,
        OR = 2,
        XOR = 2,
        CMP = 2,
        TEST = 2,
        GETXY = 2,
        ENERGY = 1,
        TRAVEL = 1,
        SHL = 2,
        SHR = 2,
        SENSE = 1,
        EAT = 0,
        RAND = 2,
        RELEASE = 1,
        CHARGE = 2,
        POKE = 2,
        PEEK = 2,
        CKSUM = 2
    }
}
