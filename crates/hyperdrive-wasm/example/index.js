import * as hyperdriveWasm from '@delvtech/hyperdrive-wasm';

const ZERO_ADDRESS = '0x'.padEnd(42, '0');
// const MAX_U256 = '0x'.padEnd(66, 'F');

// const MAX_BUDGET = BigInt(MAX_U256).toString();
const mockPoolInfo = {
  shareReserves: '50000000000000000000000',
  bondReserves: '151005848028396475250000',
  sharePrice: '1001342906360984271',
  longsOutstanding: '0',
  shortsOutstanding: '0',
  longExposure: '26169526102456856203830',
  shareAdjustment: '0',
  longAverageMaturityTime: '0',
  shortAverageMaturityTime: '0',
  lpTotalSupply: '49999999000000000000000',
  lpSharePrice: '1001342906360984270',
  withdrawalSharesProceeds: '0',
  withdrawalSharesReadyToWithdraw: '0',
};
const mockPoolConfig = {
  baseToken: ZERO_ADDRESS,
  governance: ZERO_ADDRESS,
  feeCollector: ZERO_ADDRESS,
  fees: {
    curve: '100000000000000000',
    flat: '500000000000000',
    governance: '150000000000000000',
  },
  initialSharePrice: '1000000000000000000',
  minimumShareReserves: '1000000000000000',
  minimumTransactionAmount: '1000000000000',
  timeStretch: '31124187940342208',
  positionDuration: '31536000',
  checkpointDuration: '86400',
  linkerCodeHash: '0x2b287bf09de636779ceb93a28afed387376a9bdabfcf1470369558a3dc67accc',
  linkerFactory: ZERO_ADDRESS,
  precisionThreshold: .5e18.toString(),
};

async function main() {
  hyperdriveWasm.initSync(hyperdriveWasm.wasmBuffer);

  const baseAmountRequired = hyperdriveWasm.calcOpenShort(
    mockPoolInfo,
    mockPoolConfig,
    10e18.toString(),
  );
  console.log('baseAmountRequired:', baseAmountRequired);
}

main();
