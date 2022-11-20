# Command `key`
```
Calculate the public key/SS58 address of the SS58 address/public key

Usage: subalfred key [OPTIONS] <PUBLIC KEY/SS58 ADDRESS>

Arguments:
  <PUBLIC KEY/SS58 ADDRESS>
          Public key/SS58 address

Options:
      --type <TYPE>
          Key type

          [possible values: pallet, parachain, sibling]

      --network <NETWORK>
          Network name

          [default: Substrate]

      --list-all
          List all the networks' addresses

      --show-prefix
          Show network(s) prefix(es)

      --json-output
          Enable JSON output

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Example
### Calculate the public key
```sh
subalfred key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
public-key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
Substrate 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```

### Calculate the SS58 address
```sh
subalfred key 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```
```
public-key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
Substrate 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```

### Calculate the pallet address
```sh
subalfred key --type pallet "py/trsry"
```
```
public-key 0x6d6f646c70792f74727372790000000000000000000000000000000000000000 PalletId(py/trsry)
Substrate 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
```

[Subalfred] will detect if this is a pallet address automatically.
```sh
subalfred key 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
```
```
public-key 0x6d6f646c70792f74727372790000000000000000000000000000000000000000 PalletId(py/trsry)
Substrate 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
```

### Calculate the sovereign address
Sovereign address on relaychain.
```sh
subalfred key --type parachain 2000
```
```
public-key 0x70617261d0070000000000000000000000000000000000000000000000000000 ParaId(2000)
Substrate 5Ec4AhPUwPeyTFyuhGuBbD224mY85LKLMSqSSo33JYWCazU4
```

Sovereign address on sibling chain.
```sh
subalfred key --type sibling 2000
```
```
public-key 0x7369626cd0070000000000000000000000000000000000000000000000000000 SiblId(2000)
Substrate 5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8
```

Note that, if you are on Moonbeam-link chain, remove last 24 zeros from the public-key.
```sh
public-key 0x7369626cd0070000000000000000000000000000000000000000000000000000 SiblId(2000)
public-key 0x7369626cd0070000000000000000000000000000 SiblId(2000)
```

[Subalfred] will detect if this is a sovereign address automatically.
```sh
subalfred key 5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8
```
```
public-key 0x7369626cd0070000000000000000000000000000000000000000000000000000 SiblId(2000)
Substrate 5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8
```

[Subalfred]: https://github.com/hack-ink/subalfred

### Option `list-all`
```sh
subalfred key --list-all 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```
```
public-key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
BareEd25519           NTG3mn5om1nwD45xx8aUTR3pefA8zEVwfES4dAcw1JMjNgm
BareSecp256k1         5MwePTSDBPzsBZEeCFKXaW43g4NvLKRKVHPgfxtfTD5rzTug
BareSr25519           AswXnhH3BGLd6FA9tNXietCXgNa2cyTZn8AqFt21J7PAnpZ
DICO                  6MpEyNrF2Jk8k9HHFa7mMWiJ8uort9iXNgvzpoKg5m2jonV1
Eggnet                oVRYSJ9KxJzim3FzSpeAMPBJpmQYP6wkEkHz5erKyhFmc3PUL
ICE                   npQjXEEdK3EkZyJp4gUC4jqKg4gdQeHYizXNk6bsFevGx1Dos
KICO                  6G2aiPJqe1sQb5spM3EjycSsVRfZpy61BkNsD7gNcuSFXSgN
SNOW                  ni2MJgjyWScn3yYvNiPtYaAJUhdUb8ZFQvKvLirhz2CDjzTtu
Turing                cU8jCBoMnvw9xNB3HoVoB4bPFwaLQTjVSBxarreqv8m5UubMC
acala                 24xsriZuVo8t2ctLJgHk7mH2M3fDXGe9GF7MNQfjAzTmhcR3
ajuna                 aUtUKdw182tzTyNnVgZijvnDTU9An54EGKajDGD3AM9JHr8Ci
altair                kAMF48Seoe2NLpgBpERuPiUhaBVqPuKM5kG22BGtNkgC5UF25
amplitude             6kxt1M1rZUF3NNu8shdrs7mzirP36tEc8T9XHXtswBQgvnbM
apex                  rzg96q3EEBeHKBq4PoA9jZo3RGoqMuGNUzGTXt9NHs3zsKZNg
ares                  4Tri4XZain8Kn2bU3TQKBPfDrh6GqfkdnpQW8p6xHWjUTQsr
astar                 a2aZkrtaLmFFKs1n1tdEFwu7cwkFMVYKYLhHzTDriVLJ5GZ
aventus               cLx3haNNmFzBd7wutbATTsiEfQe3EWY9xcEhQZSYEoCGNeAUs
bajun                 bUMvGXNsMQUm7wBohPESRRqQDjdY9gcv4yMpqduUb3fo9xRzt
basilisk              bXkghK7H4UNVqFBZUBLMEGUSfi8Q9YrKmPLfJZ9Lpnk1jXLSg
bifrost               fpEpkQHxddyQPGUgYmecADKm763JY84WUtpug6XKa5paJMM
calamari              dmy2WdnrD23rM4nVrA8PE3TWD8M1tB231YoceS7ck2cz4nvbM
centrifuge            4fS2aWePVMsn69QPrXAMwCC59fNrx31gAhWmNBPZDDvT24fC
cere                  6TbuENPeQbcruCgkA6znjQyinPx9wLM3ZdV8SUxyYcdE61cs
cess                  ce7TA31W7N2cdz3VgNjmYphyka6dox4TV41yzP1A2SGYF3pYk
cess-testnet          cXj4wVWrJmQe7zHbzQfU2f2xZD3UzSLAAypXb1FzkoYV32xMV
chainflip             cFMxm6p4BjhCvNfFusMC8jFLsDU2mSGtp3Zg47AXNsJy4tDoM
chainx                5TjJeSycZgsbLce76nCYxQKUKYXDPW3qgDwpHeXxv4gMGnqJ
clover                jHGBKnVSNs4aCrd4JVqVEP6Y1H4cqoUxbAgNoBKcDgskRNmg8
composable            64TGCQE2sS7xGy4tXyUhDov2CTNyibpxosGbykPmhCFGxk6n
contextfree           a7SDW4dbSQLxCPkokx6adk336WpRZGerMmMcyzFmRoJTTLqrd
cord                  3xvQmZrZoKkgzja9Wo1CHtL6dGNoZkc2r7dr4PtSyEm342uB
crust                 cTLRv7s2ZrcA97hoaZEkz3PFrmhC42GTGgS9owBhWRvKaexfe
dark                  2mUVffMjBqFy82iaeQSum58zsSfGuZ3napzGgCAqQydBg7vY
darwinia              2sG9veu8a88hH683YwKw8yQRWvoZxjgJmmYQHsp8sqDfxHHW
datahighway           4N53oY2BLVFbcyC18vXHoVPoDCwynV87bsrNX8Tepf8zBJfV
dentnet               dx9Knzz5LDj4UzmmAYS7eZNdZ3qbsmiG6nk83BqDTFqdoVwGm
dock-pos-mainnet      3GQnxd4k7HdbuKjuB4r2eaU86sNkBUCPXXkvkcPLjFbd5YU5
edgeware              mbu5jwhLvWhZSfwb5efz4UkQbELMikahRSxXMjpnRgJrcEc
efinity               efTEt1iEFLB9Pf5FDWrKUtSjUciocVawat9UPtoU6wqog1dGD
equilibrium           cg7CMCrLB3r7B7DaxVPN2NjJFVoVh3j3tpq4cgg23hNRzgFAa
fragnova              fRkmknCdFyMU718sBfEv2YYp9aAJArxWgdmSh4H1nXVpAy4qJ
geek                  XWGZATcKd9Zw3mpuxmVoefp8KNhn57zuozNL4ukSHKrciyPew
genshiro              cZip8fMgNTE8f7ThGXK4WD4H48kLsXzkakdcDJvrn4eNnfXNT
gm                    gMY6J6wTa8AssBmvDrD5q5GABvhNfSjuEJk3Tfgf56PJzfGj3
heiko                 hJKHS1bhv7u2tw11u6W3sNzAUk5t6aMays8BYRvjNHpkhAqHG
hydradx               7MgqZJGGsDVQJjKvJtv18XNYbmEoRz1jG6UJydkgiJyccwqp
integritee            2NKrdhC7efm4Vo6j2GvpFU5JHW66gpXhq4mkDTbdZZFEYdqE
integritee-incognito  hdUS5e5fKukxSvGgxzixRt1E4qFLZ7YUv5iYkZADBBzvKCfaw
interlay              wdBbAQmEBQZqDUnhj7sUehkhvTYeebSo8bPErUT1Wr572f6YH
joystream             j4VQthW8mfpdAs7GvZgtC3kVcYxKCn2My2HTzRqHgRRe1MgG8
jupiter               3fZRzbEMeT8WXZMkoCN8ABXpgowvQCiUHHyTDLxYafyaCmYh
kabocha               3mM6Famm2k1EgcmDhjF9Y5oFLJ6DTPLzUEXaq2br3Xa4VG9o
kapex                 tsY1kqQw6V4UHasRVx1veXwC2PBrAnDLLnSrn6r1n1wirN28x
karura                sPZLjV6jDPRiW5QVcXhMxkB45NdQuP6tN1693P8FHGo8zCJ
katalchain            UEvJmKVC3tX6GTYsV1brMgUU8oTCAs28bnZgJovPrtr1t9S
kilt                  4s1M6VjCFwdEQGDKfavQgzivSdfT4QGiYad2bYgA8w7RauZQ
kintsugi              a3eJSEw27oHiaTXS8Xs9jPnvwxNzPBgvabcZzcHKzCorveaqE
kulupu                2fgqQfpKoYPExyK7jsZtPAsaDxWyrNRGPtS94WXXx82hPqRq
kusama                GfbnnEgRU94n9ed4RFZ6Z9dBAWs5obykigJSwXKU9hsT2uU
laminar               2AkY7i7Jt61cBgHoDDAmVfYSzXoWaTGfTBfUz6K2dr4FyzaL
litentry              4AVjHYwNZuW9JrP5KrmF3grwvEfPg7s5Dzk7HmB3twx1cPDT
litmus                jcRKyQyPnevVkqtjNQ4Pnt7bbNE5JLfrXPGk1JZ62b3v3R4u2
manta                 dfaeJ6JCQRRsq52cAC45hsnV1mHs4fHjhUcAF4NTUPtvrnT4s
mathchain             4xo1MVGbeEVxZKcna7oS4tzM67ok7auEjXBADEKTbnhurwn7
mathchain-testnet     54afcUp12XNgiP2FUegTSoFmjbx3AmXkvTjHpuxm4eJQ9WvS
moonbeam              VducokCKmmc7iA5aB9JRrbBJqVkUjWy6Z8thUmU1JVEsBLJ47
moonriver             VkJ12HgyaNE6E9qTs7NjNkrL2rodZ2hPsD69t9DAa7xvPM3Ah
neatcoin              5rswgR9E6rNVxrFxiuieU1PAuV6PcEZvRzALkP7AmV4JPvtN
nftmart               nmv42mS6ERxtQ4qhU3ACUJJg3uZ44GhoiH1XTiWnj5YQky5bG
nodle                 4mDgqWBnsekWFCorm43PK6TVo9XA1DeCMe4tys2rg5WwJUjp
oak                   6AEvTPmSFizgS2UMSWMibiBSqwXGmnTUzopjbS35A3qmF5bR
origintrail-parachain gJqqV89qgkKGEyBzhPqLBsvyiUbWixnuBDM5v4EHwbJFq4Z7Z
p3d                   d1GLzqLHaqi2j6VG2PcGaskMqaxx9auwq3RRpouVrbYbciRnp
p3dt                  d7ejDNpwPSL1F6F9iMga73RP2x26y6eF97ctEBef8EGepiwZK
parallel              p8F2qgE7j8MSxfvGd36n7ihSdFTJtMa6JMNQXg4e5YnBXtbnX
peaq                  rtHktHYaRb2JoC5Ahq5rDQ82DukgYPY5Av518WQD2EKwfJtzL
pendulum              6fBDkMUTBBNKDKVfyAkqVDWa5NEk3hc5wWbPfrFaUKpCeWGr
phala                 44i52ZPyBcdR9nycRKtDfnbXGkX6cwEZ34Byg5XkS6MXL68i
picasso               5xfbwQgdV9FE7ufRdSbfquebYyEgfRCScviUN4kUELengQY6
pioneer_network       WWbXu4BNb1QQwVu5qFmKvQxQhpF87Q4xxxi8BJgiFhcvtJhmy
poli                  5ANKsUMQQpFQsSRiPBZUphXCP66LDxAH7QHRSbc4XVttRfvK
polkadex              esprg4jNEzFbX2MQjpsQRiBiAktY5JK28GnAgBZDSMsY8uq69
polkadexparachain     ezDEtcE23asa327JRnwhwsrjN7wgtp3KSLyd5ZJNhzbbLvKwY
polkadot              156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y
polkafoundry          g65543AY5Z5KCygDKTgj9YawKkVD5wLJZ4xB7JjyQKr9R3Dz3
polkasmith            fyggqVftGxTLgyvKdVcRdNuv8PS4GRc1Ezkihvzp8h86D2Yxp
polymesh              2GYCNheiGNtLLjhG7k3nsZose1wodduBe8DcbmxL6hekGSn8
pontem-network        gkPNMJ8Su8oAJxDZTG8YGYd4Vvp811i6RW8uXZCw28CUf7N6c
quartz_mainnet        yGGpnkEDQfqKtKjEpHFrHZY9tNxvsu6sAMmBynpuT6xmis9QJ
reserved46            5fJdAS4RLGd3ejT2uqxbiCrKcWooVsJt4745X1pZqmsKqDd7
reserved47            5m6HRRbpiZVmonrVpNqd677kFzx6Z3wQF3cD8hTsJdTp7q5Q
reynolds              yBDbj2W7WG9sZUsQ9Qijs1bhZWvU61d5JZDkj2Ri8sHRFbo
robonomics            4GHPYYUmxCNsTunYEPeGRb8NZiogjJVbQwJEuSpMMoYVtpSv
sapphire_mainnet      pxLUnGR64Nb1HxDGDbioZj3dSmLEcoubBGjRpDWdCivnZYW8r
shift                 3NCTDcc9VaWL4P9N5bj42UjYkMX3EepuiUK4NJ2eC7C7MwEw
social-network        xw7g97kFzsyQLLTZkP2wj4X6JHoURMuyE9ApmfbReCnc6q2BN
sora                  cnVaZkLyyeU5h6yUeTTfYYQKSrreWZTMCu2X24RBKL6VCh161
sora_kusama_para      oFFkmbGShjgXYrvYaD3EymDYUzWEdJvLKxjKSHo1DwqLN5eiM
stafi                 34qUSdywLht9bCvyN15ytmwGou6A56wM9eefXF6joYQeWpZG
subsocial             3s8kWaKAR2sxqgAgcG8Auz4fynEWWZyWfB5iSiF9WPAYmTxu
subspace              sufAN5qjKG8hG7zsijsHM5JG4TTmW3ais9idFa5sAj6jnm9Cg
subspace_testnet      stAMUCxcGVmaMn7om9t5zQnGgmBd6DqH4KcKHLqDenGpUYRFG
substrate             5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
synesthesia           2ZuB9gGvRFWWouueqLgs1Gc9aUNgoBnkCwt1SptEVGSD7FJ6
tidefi                fhDnzqyYkYT2mDEa63mGi5E31kNTkNN7MsZK3RDhTJ2ykbHKr
tinker                i51xwp4GYJErWuJFis2AWYhJrHTwqATgANWNN48rFiu99FPRH
totem                 2U7WtgjX2xdnerWBvooqdNLivzEPk1AE21Ksq9Ew2Qqiq2f9
uniarts               5ZWxuSX1wykKVg3a1K5aLJaty2fWSggMsAVwuLBGNvGqZAkM
unique_mainnet        unicLcCWzL8dgqA3khnpJoEwWrp4u2twmKn8m29oFohekNWVg
vln                   4ZeNKX6z7513w5zvwzHLZHveWBEZtrP9ykxdkVkFkNKxjh7r
xxnetwork             6ZPZVMw3ntVb4G6D4dsp7KF9Rt6SzWyZka3G4AcH1UDiNL2o
zeitgeist             dE37RvKbC2wym613QKksdD6QEK5FncNYTBpLdZPpPrzi2jYtt
zero                  3Tz7Uc9YssP4DSYpz8c5QNzyPqfLHqTRuQsByyfwexnbeM1M
zero-alphaville       3ZmmjbgxGAFnNVxHtfV6nHGQ3KodM25x6MRKbfKF7pP5vUpv
```

### Option `--network`
The network parameter is case [insensitive](https://github.com/paritytech/ss58-registry/pull/106).
```sh
subalfred key --network polkadot 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```
```
public-key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
polkadot 156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y
```

### Option `--show-prefix`
```sh
subalfred key --show-prefix 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```
```
public-key 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
Substrate 42 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP
```

### Option `--show-prefix`
If you don't have the `jq` command, then just ignore that.
I only use it to beautify the output here.
```sh
subalfred key --json-output 5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP | jq
```
```json
{
  "addresses": [
    {
      "address": "5G9z8Ttoo7892VqBHiSWCbnd2aEdH8noJLqZ4HFMzMVNhvgP",
      "network": "Substrate"
    }
  ],
  "public-key": "0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747",
  "sub-seed": ""
}
```

### Option `--type`
See [Calculate the pallet address](#calculate-the-pallet-address) and [Calculate the sovereign address](#calculate-the-sovereign-address) sections.
