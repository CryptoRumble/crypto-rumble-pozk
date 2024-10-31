// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { writeFile } = require('fs');

const NAME = "CryptoRumble30Verifier";
const ADDR = "0xef1e764c386ec95ed233035661dd4269be8fd8e7"; // testnet & mainnet for POZK

async function test() {
  const C = await ethers.getContractFactory(NAME);
  const contract = await C.attach(ADDR);
  const res = await contract.verify(
    "0x00000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001d0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001725732074ddc361ac18a1b5eb47ee1bae165a5db3804f31fac2bfce36f2cfdca82995d1c2b1e1a46b369ccb1497dbf0924f7c09dfeef92f4b35d0d73e1712cce00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001300000000000000000001144319084284830888510c2410ca110c4128c2318845000000000000000000028843214821142110ca2190a11088311463104621048200000000000000000004400008002dc50d0000920400c10c30c10920021400000000020000000200010202010000010201000101010201010102000101000000",
    "0x1bb7dba747be406f32f0b1177dc0dbca17fad56810f538aac896501c6d0ae03e08cb583289d632aa5d7ecc5af393e5de83c8cc613d1993ae603cb6f3a2a7f5b32840aa6baebeb47f39b6bcc07ad46e913e9b8a87c2057b59c975dc270979851312ecd5f6d2a224167a757831da9f8dd232e6075e3f4a086c1ab41e850e1d68d31c9da04788d47885d3575d5f0a0a91927a71809359e140f457387982192b8c2d17c6958a0b73c0d1f3736438a4028b42bf7cf65a9a6f3ca5fbc690a9ba0f90631303922a358a986765272bffb0bcf92ca43b27ce5274bd1a248763dc2c9bc3602c0d459c5e5a5e5395aa7e57bd31ff2ccdb14eba0b66a1ed4d374e256ee7bbfb"
  );
  console.log(res);
}

async function upgrade() {
  const C = await ethers.getContractFactory(NAME);
  const address = await C.attach(ADDR);
  const Factory = await ethers.getContractFactory(NAME);
  console.log(`${NAME} upgrading...`);
  await upgrades.upgradeProxy(address, Factory);
  console.log(`${NAME} upgraded`);
}

async function main() {
  await upgrade();
  //await test();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
