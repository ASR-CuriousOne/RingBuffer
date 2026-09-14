# Ring Buffers

## Implemented Till Now
- Fixed size stack allocated cicular queue.
- Fixed size heap allocated heap queue.

## Benchmarks

<table>
  <thead>
    <tr>
      <th rowspan="2"><center>Queue Type</th>
      <th colspan="3"><center>Average Latency (in ns)</th>
    </tr>
    <tr>
      <th><center>PushPop</th>
      <th><center>Push</th>
      <th><center>Pop</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>FixedSizeStackCircularQueue (2 ^ 16 Elements)</td>
      <td align="center">1.20313843</td>
      <td align="center">1.474888885</td>
      <td align="center">1.093576202</td>
    </tr>
    <tr>
      <td>FixedSizeHeapCircularQueue (2 ^ 16 Elements)</td>
      <td align="center">0.77667247</td>
      <td align="center">0.8497384644</td>
      <td align="center">0.7437602081</td>
    </tr>
  </tbody>
</table>

## Observations
- General 

> Constraining capacity to powers of 2 reduces latency of each operation by roughly 0.4 ns cause expensive integer division is replaced by cheap and masking.

- Fixed size stack allocated circular queue.

> Stack is not ideal for this especially if large capacity is required. It is actually slower than heap allocated one. Looking at assembly its primarily because LLVM cannot prove that array writes would not overwrite other stack variables so it is forced to read from caches again and again. Also cache is likely getting evicted very often when cpu reads other variables like head and tail (cache thrashing its called).

> Also if the capacity is set to 256, there is no and operation either! Compiler just uses a 8 bit register with unsigned arithematic(lol).

- Fixed size heap allocated circular queue.

> Heap allocation allows for certainity that there will be no overflow into other stack varibles like head and tail so those can be kept in registers only. Also heap is probably made for this stuff so some other optimization are also there.
