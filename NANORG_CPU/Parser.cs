using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace NANORG_CPU
{
    class Parser
    {
        List<Instruction> instructionList;

        public Parser(string filename)
        {
            instructionList = Load(filename);
        }

        private List<Instruction> Load(string filename)
        {
            var tmpList = new List<Instruction>();
            return tmpList;
        }
    }
}
