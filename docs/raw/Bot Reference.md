Your bots are comprised of different components working together:

- A [custom 16-bit microprocessor](CPU Details)
- Self-driven durable micro-wheels
- A renewable energy generator, fueled by an [industrial sludge]() incinerator
- A tile detection probe, which reports different [tile types](Tiles) when queried
- A rotatable serial bridge, for [communication with adjacent bots](Bot Communication)
- An [absolute positioning system]()

## Energy

Bots generate energy by processing sludge and placing it into their generator, creating energy from waste. Each unit of sludge generates 2000 units of energy for the bot. Bots can only carry a maximum of 65,535 units of energy, if they attempt to process sludge that would put their energy level over the maximum, the bot will be unable to process it.