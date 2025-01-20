// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./IVerifier.sol";

contract CryptoRumble30Verifier is Initializable, OwnableUpgradeable, ERC165, IVerifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 4191649219238059286526229902756779741975545340173074144522262541085518975416;
    uint256 constant deltax2 = 2940174195018222495457958255239781540339715148816708160232408608060012546038;
    uint256 constant deltay1 = 12460512717555380713460561588905152787980338573467130729780014103803464711446;
    uint256 constant deltay2 = 6978267109487003261314629604162026671534817615974166095175628347828236492439;


    uint256 constant IC0x = 21164450944429052294796660248829027688292980126612461806125455368558400706211;
    uint256 constant IC0y = 14293530455017056233918443178556263616476831852746130656832567362416594231613;

    uint256 constant IC1x = 11223668771354055575832694153922848604171540626221312242205583735556392928135;
    uint256 constant IC1y = 9428399231324787466346101101617057579524981181276588855808855322322676818129;

    uint256 constant IC2x = 11053605659970422098084022635623823719701718752809038480216213289269134022521;
    uint256 constant IC2y = 19894962434351343597561853561474841169542987101059401195655411185449339950097;

    uint256 constant IC3x = 17190891980287973944055810503815826877982642423549485101318678079034220060093;
    uint256 constant IC3y = 13958068674956021393790308962226612948988762654654903425046650339800479222205;

    uint256 constant IC4x = 14917671772695140273057636049180503577987914104389081158972530773630467471845;
    uint256 constant IC4y = 15947957857359793032118562565655288819716397261488793150738296990026692765856;

    uint256 constant IC5x = 668991439567545909810602329388753508200186661881319765265775454345919835800;
    uint256 constant IC5y = 2486842971782669678961171084880009073173618847056254147861712777573333958265;

    uint256 constant IC6x = 1036871264067478987058710586017617672371254648328575239507419365488945707905;
    uint256 constant IC6y = 17211025926518414831191137001366865466619912094690456564973148273242813189872;

    uint256 constant IC7x = 13360092789551425829479058595021677107263086046621173339412346408963769456315;
    uint256 constant IC7y = 16125527335840899537016489467074146727627047340416262483005897771822281355709;

    uint256 constant IC8x = 21206951635911149901120011634601401423576389473361026609734590430063478361487;
    uint256 constant IC8y = 14924388644046416898670145990210906652590639191916422508286132795757927176777;

    uint256 constant IC9x = 7925934084887001014458655626292688773512453678589874385532920224590040322385;
    uint256 constant IC9y = 20965129532997546693719815326348679499339667711286396038261958823082896869318;

    uint256 constant IC10x = 698173783479298453176560211977995956961975157847487633177900258170424771442;
    uint256 constant IC10y = 6428514442163858568481129934801294366922073665422752265107906208236895927277;

    uint256 constant IC11x = 1138276518480757809290271181372024448300269114542093863253996921636226746483;
    uint256 constant IC11y = 12845407919923844631603253035160029483009383315123646783329153815824852889589;

    uint256 constant IC12x = 12922036643803572657284874038295098103620061871460648719213713861768349687553;
    uint256 constant IC12y = 4704949854293889843499940381219155686936498777023830812360358553131332487614;

    uint256 constant IC13x = 2901498858715779161624200472204122630691374533627394652077568150443644208573;
    uint256 constant IC13y = 17818096939622132408606702344249133628163907093576245893742955232682064320489;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC165) returns (bool) {
        return interfaceId == type(IVerifier).interfaceId || super.supportsInterface(interfaceId);
    }

    function name() external view returns (string memory) {
        return "crypto-rumble-30";
    }

    function permission(address _sender) external view returns (bool) {
        return true;
    }

    /// show how to serialize/deseriaze the inputs params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function inputs() external pure returns (string memory) {
        return "uint256[9]";
    }

    /// show how to serialize/deserialize the publics params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function publics() external pure returns (string memory) {
        return "uint256[13]";
    }

    function types() external pure returns (string memory) {
        return "zk";
    }

    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool) {
        uint[13] memory _pubSignals = abi.decode(publics, (uint[13]));
        (uint[2] memory _pA, uint[2][2] memory _pB, uint[2] memory _pC) = abi.decode(proof, (uint[2], uint[2][2], uint[2]));
        return this.verifyProof(_pA, _pB, _pC, _pubSignals);
    }

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[13] calldata _pubSignals) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x

                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))

                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))

                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))

                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))

                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))

                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))

                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))

                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))

                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))

                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))

                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))

                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))

                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))


                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))


                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)


                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F

            checkField(calldataload(add(_pubSignals, 0)))

            checkField(calldataload(add(_pubSignals, 32)))

            checkField(calldataload(add(_pubSignals, 64)))

            checkField(calldataload(add(_pubSignals, 96)))

            checkField(calldataload(add(_pubSignals, 128)))

            checkField(calldataload(add(_pubSignals, 160)))

            checkField(calldataload(add(_pubSignals, 192)))

            checkField(calldataload(add(_pubSignals, 224)))

            checkField(calldataload(add(_pubSignals, 256)))

            checkField(calldataload(add(_pubSignals, 288)))

            checkField(calldataload(add(_pubSignals, 320)))

            checkField(calldataload(add(_pubSignals, 352)))

            checkField(calldataload(add(_pubSignals, 384)))

            checkField(calldataload(add(_pubSignals, 416)))


            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
         }
     }
 }
