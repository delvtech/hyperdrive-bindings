import * as hyperdriveWasm from '@delvtech/hyperdrive-wasm';

const ZERO_ADDRESS = '0x'.padEnd(42, '0');
const MAX_U256 = '0x'.padEnd(66, 'F');
const MAX_BUDGET = BigInt(MAX_U256).toString();

const examplePoolInfo = {
  shareReserves: '10000000000000000000000000',
  shareAdjustment: '0',
  zombieBaseProceeds: '0',
  zombieShareReserves: '0',
  bondReserves: '10217899519533796120000000',
  lpTotalSupply: '9999990000000000000000000',
  vaultSharePrice: '1000000000000000000',
  sharePrice: '1000000000000000000',
  longsOutstanding: '0',
  longAverageMaturityTime: '0',
  shortsOutstanding: '0',
  shortAverageMaturityTime: '0',
  withdrawalSharesReadyToWithdraw: '0',
  withdrawalSharesProceeds: '0',
  lpSharePrice: '1000000000000000000',
  longExposure: '0',
};
const examplePoolConfig = {
  baseToken: ZERO_ADDRESS,
  governance: ZERO_ADDRESS,
  feeCollector: ZERO_ADDRESS,
  fees: {
    curve: '100000000000000000',
    flat: '500000000000000',
    governanceLP: '10000000000000000',
    governanceZombie: '100000000000000000',
  },
  initialVaultSharePrice: '1000000000000000000',
  minimumShareReserves: '10000000000000000000',
  minimumTransactionAmount: '1000000000000000',
  timeStretch: '44463125629060298',
  positionDuration: '604800',
  checkpointDuration: '3600',
  linkerCodeHash: '0x'.padEnd(66, '0'),
  linkerFactory: ZERO_ADDRESS,
  sweepCollector: ZERO_ADDRESS,
  vaultSharesToken: ZERO_ADDRESS,
};

async function main() {
  hyperdriveWasm.initSync(hyperdriveWasm.wasmBuffer);

  const spotRate = hyperdriveWasm.getSpotRate(
    examplePoolInfo,
    examplePoolConfig
  );
  console.log('spotRate:', spotRate);

  const maxLong = hyperdriveWasm.getMaxLong(
    examplePoolInfo,
    examplePoolConfig,
    MAX_BUDGET,
    '90844806244066488'
  );
  console.log('maxLong:', maxLong);

  const baseForMaxLong = hyperdriveWasm.calcOpenLong(
    examplePoolInfo,
    examplePoolConfig,
    maxLong
  );
  console.log('baseForMaxLong:', baseForMaxLong);

  const spotPrice = hyperdriveWasm.getSpotPrice(
    examplePoolInfo,
    examplePoolConfig
  );
  console.log('spotPrice:', spotPrice);

  // Returns: 15977080231906768517
  // const openShortPreview = hyperdriveWasm.calcOpenShort(
  //   examplePoolInfo,
  //   examplePoolConfig,
  //   (10_000n * 10n ** 18n).toString(),
  //   "0"
  // );
  // Returns: 1127088191343017879628
  const openShortPreview = hyperdriveWasm.calcOpenShort(
    examplePoolInfo,
    examplePoolConfig,
    (10_000n * 10n ** 18n).toString(),
    '900000000000000000'
  );
  console.log('openShortPreview', openShortPreview);

  const date = new Date();
  const currentTime = Math.floor(date.getTime() / 1000);
  const sixMonthsFromNow = date.setMonth(date.getMonth() + 12);
  const sixMonthsFromNowTime = Math.floor(sixMonthsFromNow / 1000);
  const feeArgs = [
    examplePoolInfo,
    examplePoolConfig,
    (10_000n * 10n ** 18n).toString(),
    sixMonthsFromNowTime.toString(),
    currentTime.toString(),
  ];

  console.log('feeArgs:', feeArgs);

  const closeShortCurveFee = hyperdriveWasm.getCloseShortCurveFee(...feeArgs);
  console.log('closeShortCurveFee:', closeShortCurveFee);

  const closeShortFlatFee = hyperdriveWasm.getCloseShortFlatFee(...feeArgs);
  console.log('closeShortFlatFee:', closeShortFlatFee);

  const openShortCurveFee = hyperdriveWasm.getOpenShortCurveFee(...feeArgs);
  console.log('openShortCurveFee:', openShortCurveFee);
}

main();
