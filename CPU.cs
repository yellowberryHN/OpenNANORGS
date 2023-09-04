#nullable enable
using System;
using System.Collections.Generic;

namespace OpenNANORGS;

class CPU
{
    private const ushort MemorySize = 3600;

    private readonly Random rnd;
    private Bot bot;

    public Instruction CurrentInstruction { get; private set; } = new Instruction();
    public ushort IP { get; private set; } = 0;
    public ushort SP { get; private set; } = MemorySize;
    public ushort[] Registers { get; private set; } = new ushort[14];
    public ushort[] PMemory { get; private set; } = new ushort[MemorySize];

    // for debugging mutations
    private Dictionary<ushort, (ushort, ushort)> mutations = new Dictionary<ushort, (ushort, ushort)>();

    public CPU(Bot bot, ushort[]? memory, Random rng)
    {
        if(memory != null) PMemory = memory;
        rnd = rng;
    }

    public void MutateMemory()
    {
        ushort pos = (ushort)rnd.Next(MemorySize);
        ushort mut = (ushort)rnd.Next(ushort.MaxValue);
        
        mutations.Add(pos, (PMemory[pos], mut));
        
        PMemory[pos] = mut;
    }

    private ushort AdvanceInstructionPointer(ushort step = 3)
    {
        return SetInstructionPointer((ushort)(IP + step));
    }

    private ushort SetInstructionPointer(ushort pos)
    {
        if (pos % 3 != 0) throw new NANORGException($"IP is not multiple of 3. IP = {pos}");
        IP = (ushort)(pos % MemorySize);
        return IP;
    }

    private ushort SetInstructionPointer(Operand pos)
    {
        return SetInstructionPointer(GetValue(pos));
    }

    private Instruction NextInstruction()
    {
        CurrentInstruction = new Instruction(PMemory[IP..(IP+3)], IP);
        return CurrentInstruction;
    }
    
    public void RunNext()
    {
        var inst = NextInstruction();
        switch (inst.opCode)
        {
            case CPUOpCode.NOP:
                Oper_NOP();
                break;
            case CPUOpCode.MOV:
                Oper_MOV(inst.op1, inst.op2);
                break;
            case CPUOpCode.PUSH:
                Oper_PUSH(inst.op1);
                break;
            case CPUOpCode.POP:
                Oper_POP(inst.op1);
                break;
            case CPUOpCode.CALL:
                Oper_CALL(inst.op1);
                break;
            case CPUOpCode.RET:
                Oper_RET();
                break;
            case CPUOpCode.JMP:
                Oper_JMP(inst.op1);
                break;
            case CPUOpCode.JL:
                Oper_JL(inst.op1);
                break;
            case CPUOpCode.JLE:
                Oper_JLE(inst.op1);
                break;
            case CPUOpCode.JG:
                Oper_JG(inst.op1);
                break;
            case CPUOpCode.JGE:
                Oper_JGE(inst.op1);
                break;
            case CPUOpCode.JE:
                Oper_JE(inst.op1);
                break;
            case CPUOpCode.JNE:
                Oper_JNE(inst.op1);
                break;
            case CPUOpCode.JS:
                Oper_JS(inst.op1);
                break;
            case CPUOpCode.JNS:
                Oper_JNS(inst.op1);
                break;
            case CPUOpCode.ADD:
                Oper_ADD(inst.op1, inst.op2);
                break;
            case CPUOpCode.SUB:
                Oper_SUB(inst.op1, inst.op2);
                break;
            case CPUOpCode.MULT:
                Oper_MULT(inst.op1, inst.op2);
                break;
            case CPUOpCode.DIV:
                Oper_DIV(inst.op1, inst.op2);
                break;
            case CPUOpCode.MOD:
                Oper_MOD(inst.op1, inst.op2);
                break;
            case CPUOpCode.AND:
                Oper_AND(inst.op1, inst.op2);
                break;
            case CPUOpCode.OR:
                Oper_OR(inst.op1, inst.op2);
                break;
            case CPUOpCode.XOR:
                Oper_XOR(inst.op1, inst.op2);
                break;
            case CPUOpCode.CMP:
                Oper_CMP(inst.op1, inst.op2);
                break;
            case CPUOpCode.TEST:
                Oper_TEST(inst.op1, inst.op2);
                break;
            case CPUOpCode.GETXY:
                Oper_GETXY(inst.op1, inst.op2);
                break;
            case CPUOpCode.ENERGY:
                Oper_ENERGY(inst.op1);
                break;
            case CPUOpCode.TRAVEL:
                Oper_TRAVEL(inst.op1);
                break;
            case CPUOpCode.SHL:
                Oper_SHL(inst.op1, inst.op2);
                break;
            case CPUOpCode.SHR:
                Oper_SHR(inst.op1, inst.op2);
                break;
            case CPUOpCode.SENSE:
                Oper_SENSE(inst.op1);
                break;
            case CPUOpCode.EAT:
                Oper_EAT();
                break;
            case CPUOpCode.RAND:
                Oper_RAND(inst.op1, inst.op2);
                break;
            case CPUOpCode.RELEASE:
                Oper_RELEASE(inst.op1);
                break;
            case CPUOpCode.CHARGE:
                Oper_CHARGE(inst.op1, inst.op2);
                break;
            case CPUOpCode.POKE:
                Oper_POKE(inst.op1, inst.op2);
                break;
            case CPUOpCode.PEEK:
                Oper_PEEK(inst.op1, inst.op2);
                break;
            case CPUOpCode.CKSUM:
                Oper_CKSUM(inst.op1, inst.op2);
                break;
            default:
                Oper_NOP();
                break;
        }
    }
    
    private ushort GetValue(Operand op)
    {
        ushort tmp = 0;
        switch (op.type)
        {
            case CPUOperType.Register:
                try
                {
                    tmp = Registers[op.value];
                }
                catch (IndexOutOfRangeException)
                {
                    tmp = 0;
                }
                break;
            case CPUOperType.Direct:
                try
                {
                    tmp = PMemory[op.value];
                }
                catch (IndexOutOfRangeException)
                {
                    tmp = 0;
                }
                break;
            case CPUOperType.Immediate:
                tmp = op.value;
                break;
            case CPUOperType.RegisterIndexed:
                try
                {
                    if (op.sub) tmp = PMemory[(ushort)(Registers[op.value] - op.offset)];
                    else tmp = PMemory[(ushort)(Registers[op.value] + op.offset)];
                }
                catch (IndexOutOfRangeException)
                {
                    tmp = 0;
                }
                break;
        }

        return tmp;
    }

    private void SetValue(Operand op, ushort value)
    {
        switch (op.type)
        {
            case CPUOperType.Register:
                try
                {
                    Registers[op.value] = value;
                }
                catch (IndexOutOfRangeException)
                {
                    // NOP
                }
                break;
            case CPUOperType.Direct:
                try
                {
                    PMemory[op.value] = value;
                }
                catch (IndexOutOfRangeException)
                {
                    // NOP
                }
                break;
            case CPUOperType.Immediate:
                // not valid
                break;
            case CPUOperType.RegisterIndexed:
                try
                {
                    if (op.sub) PMemory[(ushort)(Registers[op.value] - op.offset)] = value;
                    else PMemory[(ushort)(Registers[op.value] + op.offset)] = value;
                }
                catch (IndexOutOfRangeException)
                {
                    // NOP
                }
                break;
        }
    }
    
    private void UseEnergy(byte amount = 1)
    {
        bot.energy -= amount;
    }
    
    #region Operations

    private void Oper_NOP() // OpCode 0
    {
        UseEnergy(); // NOP does nothing, NO OPERATION, just use energy
        AdvanceInstructionPointer();
    }

    private void Oper_MOV(Operand dest, Operand src) // OpCode 1
    {
        var src_tmp = GetValue(src);
        SetValue(dest, src_tmp);
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_PUSH(Operand src) // OpCode 2
    {
        try
        {
            SP--;
            PMemory[SP] = GetValue(src);
        }
        catch (IndexOutOfRangeException)
        {
            SP = MemorySize;
            SP--;
            PMemory[SP] = GetValue(src);
        }
        
        //Console.WriteLine($"     stack {SP}");
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_POP(Operand dest) // OpCode 3
    {
        try
        {
            SetValue(dest, PMemory[SP]);
        }
        catch (IndexOutOfRangeException)
        {
            SP = MemorySize;
            SetValue(dest, PMemory[SP]);
        }
        SP++;
        
        //Console.WriteLine($"     stack {SP}");
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_CALL(Operand address) // OpCode 4
    {
        // Push IP + 3 to stack, and then sets IP to address
        SP--;
        PMemory[SP] = (ushort)(IP + 3);
        SetInstructionPointer(address);
        UseEnergy();
    }
    
    private void Oper_RET() // OpCode 5
    {
        // Pop top address on stack and sets IP to it
        SetInstructionPointer(PMemory[SP]);
        SP++;
        UseEnergy();
    }

    // TODO: fix jumps, they appear to be broken.
    private void Oper_JMP(Operand address) // OpCode 6
    {
        SetInstructionPointer(address);
        UseEnergy();
    }

    private void Oper_JL(Operand address) // OpCode 7
    {
        if(flags.HasFlag(BotFlags.LESS)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JLE(Operand address) // OpCode 8
    {
        if(flags.HasFlag(BotFlags.LESS) || flags.HasFlag(BotFlags.EQUAL)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JG(Operand address) // OpCode 9
    {
        if(flags.HasFlag(BotFlags.GREATER)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JGE(Operand address) // OpCode 10
    {
        if(flags.HasFlag(BotFlags.GREATER) || flags.HasFlag(BotFlags.EQUAL)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JE(Operand address) // OpCode 11
    {
        if(flags.HasFlag(BotFlags.EQUAL)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JNE(Operand address) // OpCode 12
    {
        if(!flags.HasFlag(BotFlags.EQUAL)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JS(Operand address) // OpCode 13
    {
        if(flags.HasFlag(BotFlags.SUCCESS)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }
    
    private void Oper_JNS(Operand address) // OpCode 14
    {
        if(!flags.HasFlag(BotFlags.SUCCESS)) SetInstructionPointer(address);
        else AdvanceInstructionPointer();
        UseEnergy();
    }

    private void Oper_ADD(Operand dest, Operand src) // OpCode 15
    {
        SetValue(dest, (ushort)(GetValue(dest) + GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_SUB(Operand dest, Operand src) // OpCode 16
    {
        SetValue(dest, (ushort)(GetValue(dest) - GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_MULT(Operand dest, Operand src) // OpCode 17
    {
        SetValue(dest, (ushort)(GetValue(dest) * GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_DIV(Operand dest, Operand src) // OpCode 18
    {
        try
        {
            SetValue(dest, (ushort)(GetValue(dest) / GetValue(src)));
        }
        catch (DivideByZeroException)
        {
            // ignore, act as NOP
        }
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_MOD(Operand dest, Operand src) // OpCode 19
    {
        try
        {
            SetValue(dest, (ushort)(GetValue(dest) % GetValue(src)));
        }
        catch (DivideByZeroException)
        {
            // ignore, act as NOP
        }
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_AND(Operand dest, Operand src) // OpCode 20
    {
        SetValue(dest, (ushort)(GetValue(dest) & GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_OR(Operand dest, Operand src) // OpCode 21
    {
        SetValue(dest, (ushort)(GetValue(dest) | GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_XOR(Operand dest, Operand src) // OpCode 22
    {
        SetValue(dest, (ushort)(GetValue(dest) ^ GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_CMP(Operand op1, Operand op2) // OpCode 23
    {
        var val1 = GetValue(op1);
        var val2 = GetValue(op2);
        flags = BotFlags.NONE;
        if (val1 < val2) flags |= BotFlags.LESS;
        else if (val1 == val2) flags |= BotFlags.EQUAL;
        else if (val1 > val2) flags |= BotFlags.GREATER;
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_TEST(Operand op1, Operand op2) // OpCode 24
    {
        var val1 = GetValue(op1);
        var val2 = GetValue(op2);
        flags = BotFlags.NONE;
        if((val1 & val2) == 0) flags |= BotFlags.SUCCESS;
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_GETXY(Operand argX, Operand argY) // OpCode 25
    {
        SetValue(argX, bot.x);
        SetValue(argY, bot.y);
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_ENERGY(Operand dest) // OpCode 26
    {
        SetValue(dest, bot.energy);
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_TRAVEL(Operand dir) // OpCode 27
    {
        var dir_val = GetValue(dir);
        if(bot.energy < 10)
        {
            flags &= ~BotFlags.SUCCESS;
            UseEnergy();
            AdvanceInstructionPointer();
            return;
        }
        var success = bot.AttemptTravel(dir_val);
        
        if(!success)
        {
            flags &= ~BotFlags.SUCCESS;
            UseEnergy();
            AdvanceInstructionPointer();
            return;
        }
        flags |= BotFlags.SUCCESS;
        UseEnergy(10);
        AdvanceInstructionPointer();
    }

    private void Oper_SHL(Operand dest, Operand src) // OpCode 28
    {
        SetValue(dest, (ushort)(GetValue(dest) << GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_SHR(Operand dest, Operand src) // OpCode 29
    {
        SetValue(dest, (ushort)(GetValue(dest) >> GetValue(src)));
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_SENSE(Operand dest) // OpCode 30
    {
        var type = bot.GetElement();
        SetValue(dest, type);
        if(type != 0) flags |= BotFlags.SUCCESS;
        else flags &= ~BotFlags.SUCCESS;
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_RAND(Operand dest, Operand max) // OpCode 31
    {
        var result = (ushort)rnd.Next(GetValue(max));
        SetValue(dest, result);
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_EAT() // OpCode 32
    {
        if (bot.energy > 0xFFFF - 2000)
        {
            flags &= ~BotFlags.SUCCESS;
            UseEnergy();
            AdvanceInstructionPointer();
            return;
        }
        var id = bot.Consume();
        if(id < 1)
        {
            flags &= ~BotFlags.SUCCESS;
            UseEnergy();
            AdvanceInstructionPointer();
            return;
        }

        bot.energy += 2000;

        flags |= BotFlags.SUCCESS;
        UseEnergy();
        AdvanceInstructionPointer();
    }
    
    private void Oper_RELEASE(Operand amt) // OpCode 33
    {
        var amt_val = GetValue(amt);
        if (bot.energy < amt_val)
        {
            flags &= ~BotFlags.SUCCESS;
            UseEnergy();
            AdvanceInstructionPointer();
            return;
        }
        if(bot.Collect(amt_val)) flags |= BotFlags.SUCCESS;
        else flags &= ~BotFlags.SUCCESS;
        
        UseEnergy();
        AdvanceInstructionPointer();
    }

    private void Oper_CHARGE(Operand dir, Operand energy) // OpCode 34
    {
        // TODO: implement
    }

    private void Oper_POKE(Operand dir, Operand offset)
    {
        // TODO: implement
    }

    private void Oper_PEEK(Operand dest, Operand offset)
    {
        // TODO: implement
    }

    private void Oper_CKSUM(Operand start, Operand end)
    {
        // TODO: implement
    }
    
    #endregion
    
    [Flags]
    public enum BotFlags
    {
        NONE,
        SUCCESS = 1,
        LESS = 1 << 1,
        EQUAL = 1 << 2,
        GREATER = 1 << 3,
    }

    public BotFlags flags;

    public string FlagRender()
    {
        string a = string.Empty;

        if (flags.HasFlag(BotFlags.EQUAL)) a += "e";
        else if (flags.HasFlag(BotFlags.LESS)) a += "l";
        else if (flags.HasFlag(BotFlags.GREATER)) a += "g";

        if (flags.HasFlag(BotFlags.SUCCESS)) a += "s";
        
        return $"{a,-2}";
    }
    
    public string Disassemble()
    {
        var inst = CurrentInstruction;
        // string.Format("// [{0}] = {1}", inst.G)
        // "cmp r0, 2000                        // (r0 = 9995)"
        var buffer = string.Format("{0:D4}  {1,-34}  {2}", IP, inst.ToAssembly(), inst.op1 != null ? string.Format("// ({0} = {1})", inst.op1, GetValue(inst.op1)) : "");
        return buffer;
    }
}