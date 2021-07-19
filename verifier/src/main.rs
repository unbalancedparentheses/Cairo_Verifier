mod uint256_ops;
mod prime_field;
mod verify_merkle;
mod verifier_channel;
mod horner_eval;
mod fri;
mod memory_map;

use num256::Uint256;

fn main() {
    println!("Hello, world!");

    let proof = vec![
        uint256_ops::get_uint256("BEB1B8B9A8477DF9D44CC95A9CD5A9194C16D0238E7EC364F72487DF5C4A06"),
        uint256_ops::get_uint256("6D39056A6D89CCC48465706A45EE5937A2750962FBE839942F110B31C84D2E8"),
        uint256_ops::get_uint256("64F8F4A784F6EFCBD02E7AC435081222486EBE2D1B595684400E38B9DA76EBE"),
        uint256_ops::get_uint256("1BBFE3131F065DB9BDC25EF748602B21433EBDCA39E1A43A5C1634167B78B1B"),
        uint256_ops::get_uint256("02c700808b8247a139fc7a01493cd7517e315e48f84d0ee998870e70e554a821"),
        uint256_ops::get_uint256("700E2121B2FEB79631B2049F22B2A089D9940EDD91774F9E794431417B1B826"),
        uint256_ops::get_uint256("2717DA2FC3ECF7B7487CC13B8A26F855C15B9E21306E34016A6F77219A6B713"),
        uint256_ops::get_uint256("250072BD637E851834347BF898498B7E3C13C12176D0E64D7D5279FD19AA0BC"),
        uint256_ops::get_uint256("F98C6F3C8AB41ECC08DCDA66EE52F6748C3ED485F75292C42C9EF3487BE2E9"),
        uint256_ops::get_uint256("21CC220F4D78E164EF0C5B0C4B386E2D15D1C98FC91AF76186007DE5778B69"),
        uint256_ops::get_uint256("64C3C89230E7A8FE737C8EFDC62431195D8D1714AF1CD4392594B57DD480023"),
        uint256_ops::get_uint256("727089973A291CFEF6690C68F69B04055158914D185C4D845396C7C02F9CBB"),
        uint256_ops::get_uint256("3BACBBE67CDD11DD8A6D68842C2A1357C870B3677248C7E0E7B6C7C804542AD"),
        uint256_ops::get_uint256("182B875BC33B2BEBA47D91318DA741D4BD4EBAE30BE00992FD0816A561C0E59"),
        uint256_ops::get_uint256("228E0CE9288C457601491E4C0F6A0F4A7949EF256ED63874265943C0D321F1F"),
        uint256_ops::get_uint256("42009DF4A10A16F4F6A99E161FC3DF4ABA130616EC02FC211E8EAACEC6BF4BA"),
        uint256_ops::get_uint256("2B898C049200BC0DE48705D723F226E22272B8ADC14C0AAA19D9A0745BE717A"),
        uint256_ops::get_uint256("5dddd5106e3159be56cce000ede416abac6a580721ca6f531fc816f6c8b43a5"),
        uint256_ops::get_uint256("1EF1CAD653F4565AA78FEF3151746264BAB97657F0EF5D8683C1BFDAD6738F"),
        uint256_ops::get_uint256("34D0800CC4CA30392991D9E7B511A8619ECC35DAB68262A60A6A751F4890DDF"),
        uint256_ops::get_uint256("41F4F2D2B2159ABA46B26017D6AFC08C2F25ADEF8314C52B745D65593D15E49"),
        uint256_ops::get_uint256("6794EF0EF42141CF1C3443C06C8F90AF84E3A348675F6260EBC448DE8A15F1C"),
        uint256_ops::get_uint256("339919B13A051B00EB70EA95BF16039FC1CEBCF957EAF631AE090BCC977A74D"),
        uint256_ops::get_uint256("11677B4C60A8568BF408614585967BB6A10B17A81E0B65CDF6497A634CA68C8"),
        uint256_ops::get_uint256("6AC8F371FA684892E3FB3BC8DDC953A0C32E890BB8A7999ADCE598E695CECF7"),
        uint256_ops::get_uint256("5B8C436CC72C6D3FCF29DF54C535A33DC9E025D2C65D091666A2238742BBFF"),
        uint256_ops::get_uint256("929851B18224352038A5456DA9E74C2FB4DA54E3611C79286F8686149A9195"),
        uint256_ops::get_uint256("202B5FB99AAFCC8A636B7080421EABE45FAC5EE8D70052166870E0CB838BF4D"),
        uint256_ops::get_uint256("4A77F3E3CBA858F4E756A14A8B32DF599C9F3D79264E83BA94D9089CF2F609"),
        uint256_ops::get_uint256("51FA5AE4C1B87374819AB70F036C5D20EEED6E98E79E54D7C4DEC30D55FD047"),
        uint256_ops::get_uint256("51E4F114769100EFFDF74A42B94BDB0D3AF059A6EF29F3E1FB57AD5093B0172"),
        uint256_ops::get_uint256("2D2F33993E98424EF719A4DFCE79EFA542E7AA437EAFDB147B79A96C2723146"),
        uint256_ops::get_uint256("178E877AF3C131CFAED78F4772AEF4F4D89BCCC18C2C6079B2CDD2F17DBA4F1"),
        uint256_ops::get_uint256("626A818C749C63D9EC70E3BBC0499BA6572E4FB4151EF2BEE44E7A76735C150"),
        uint256_ops::get_uint256("4B19F733A98A7D69B493D323B85BA33CFB23CCAD52144B2C6BCECFC8C3A72D6"),
        uint256_ops::get_uint256("494D4527405C142914F6E87A9443209B8937B0BD4E55D595903403A76678FC2"),
        uint256_ops::get_uint256("106D18C4002F3483D9BAA5D1A81E246AF100460C000000000000000000000000"),
        uint256_ops::get_uint256("ADB892B5E8AEB7EE290B861B2F6F0CEE8CCB1848000000000000000000000000"),
        uint256_ops::get_uint256("F1CC5D70B93B28D284A175B0BD5D897776E7F353000000000000000000000000"),
        uint256_ops::get_uint256("F8FDBE5952C45BE554A91B21D95233AA05915911000000000000000000000000"),
        uint256_ops::get_uint256("86A3E788D52E4AB0DE28500CB1E146EC79CA9E8F000000000000000000000000"),
        uint256_ops::get_uint256("BB21F1F719603928B7E52B8D85CED5864A47B6DC000000000000000000000000"),
        uint256_ops::get_uint256("1172AE9710C1422BFAF17C0059ADE6BF8692B6F0000000000000000000000000"),
        uint256_ops::get_uint256("FF5F7DF9FEF047EB26C4D8A0378F240771A86BD1000000000000000000000000"),
        uint256_ops::get_uint256("D686F17593D61F6616CF6AE5FF05EF739724EBB4000000000000000000000000"),
        uint256_ops::get_uint256("C20BF718DBEA35A8DC0F110EBACF6A2A4E4D7724000000000000000000000000"),
        uint256_ops::get_uint256("C139C853F0C9FC6DD1005D59F5B27F1D6C2E39C8000000000000000000000000"),
        uint256_ops::get_uint256("E097935D108368F333A681E44449C4F77E039D2C000000000000000000000000"),
        uint256_ops::get_uint256("76F7387E37116F084F45045725C48FC43772DAC7000000000000000000000000"),
        uint256_ops::get_uint256("50E9506407B4FBA39815A053EEDAB4CE9521DE9F000000000000000000000000"),
        uint256_ops::get_uint256("F487C52E5D60BAAADAB89670967F960D0B0D9C96000000000000000000000000"),
        uint256_ops::get_uint256("B5348E1191C96E11EA81453FAB4D19DA994C76B4000000000000000000000000"),
        uint256_ops::get_uint256("E567E13F52B09FCE2F8F89FAB28DEF58905C240E000000000000000000000000"),
        uint256_ops::get_uint256("8CEDE3497DF5130EB975FF5D69113AD85A5D7AF4000000000000000000000000"),
        uint256_ops::get_uint256("CB3ACD79075359115A271247A405CE7698C761EC000000000000000000000000"),
        uint256_ops::get_uint256("26CC8E2E340F5C41AD1F316575708F309EF0EBB8000000000000000000000000"),
        uint256_ops::get_uint256("517DDB9096B892E616ADE1B6EA706590420C0405000000000000000000000000"),
        uint256_ops::get_uint256("440C117F0913F4C2722C533F3430A535BBCD5A49000000000000000000000000"),
        uint256_ops::get_uint256("215FF6449858043C9BAC1E83F03BE57E50B991DD000000000000000000000000"),
        uint256_ops::get_uint256("9772FBFF66A1B01BCDD9E424AE4344E6C925F098000000000000000000000000"),
        uint256_ops::get_uint256("B31E8ED91F27E66EC175C116A3E9093CF7F64D7A000000000000000000000000"),
        uint256_ops::get_uint256("F604CE59F41180E4ED57E43E2ACAD3965819E4A1000000000000000000000000"),
        uint256_ops::get_uint256("AA102CA2FB6E727A3D513D705E565B20F7DA44B0000000000000000000000000"),
        uint256_ops::get_uint256("52321A910C3BA8A9C66EC26C38884A313D0A5BD2000000000000000000000000"),
        uint256_ops::get_uint256("39C323743F03325B56A8D67D5AA49D657C2B0968000000000000000000000000"),
        uint256_ops::get_uint256("AF10496AE254309884DA6E4698539D24A29BA6FF000000000000000000000000"),
        uint256_ops::get_uint256("d895fbeb266d323cff4d94e1260045bdc4750fd000000000000000000000000"),
        uint256_ops::get_uint256("B0FAE3BC4EEEB03848059607CB7E75D92D4209BB000000000000000000000000"),
        uint256_ops::get_uint256("A17832669BF07D342DFC5F2BFEF61F6DF339F1ED000000000000000000000000"),
        uint256_ops::get_uint256("DEA0D82B8AA3CF1446CD95237062FBBE0E4A93A9000000000000000000000000"),
        uint256_ops::get_uint256("CC322D149F0EA1264D0954FDE18138C326E2782A000000000000000000000000"),
        uint256_ops::get_uint256("65D027918110B69EA02527CDC5A3D88A26DEBE89000000000000000000000000"),
        uint256_ops::get_uint256("2F62719B66F87F57E7718D6482FEBE2AAA6BA043000000000000000000000000"),
        uint256_ops::get_uint256("6A2A91EBC751F75B655A7988DAA471393D976BB6000000000000000000000000"),
        uint256_ops::get_uint256("D3E82DB9B4B932EF860196F02AA21594B22065E0000000000000000000000000"),
        uint256_ops::get_uint256("CD95B4629D9E44B926D600A02D667E562F99A218000000000000000000000000"),
        uint256_ops::get_uint256("8E7D545047864AEB9CC70865E77350BBEA073E94000000000000000000000000"),
        uint256_ops::get_uint256("284B6391212D971BFF9761625058D6E86D893D50000000000000000000000000"),
        uint256_ops::get_uint256("9AA0661B605070609086D4522E0E8303010419FD000000000000000000000000"),
        uint256_ops::get_uint256("561EAF9809D66A3EFFBBA01505098689EECD519E000000000000000000000000"),
        uint256_ops::get_uint256("D6EF296EDFBF094C4D3F16F98F52170FF3D19CB8000000000000000000000000"),
        uint256_ops::get_uint256("A922D80469F3013619AEA0AF68ABC3E32D711265000000000000000000000000"),
        uint256_ops::get_uint256("AE6303E974129A6B1B941D0762337B1CAE6B0911000000000000000000000000"),
        uint256_ops::get_uint256("B3BC2F14282CBA8548833F13C107F25B34F32A4D000000000000000000000000"),
        uint256_ops::get_uint256("E500EEE1383769F06FD90BB8990C4C7F7EBCAA21000000000000000000000000"),
        uint256_ops::get_uint256("E52C56EC96E20BBAC73CCF0928E66041FA0CF619000000000000000000000000"),
        uint256_ops::get_uint256("CE5EC5BEB8B12614A0C82D9FE2A6249DC60BAAC0000000000000000000000000"),
        uint256_ops::get_uint256("FD8AEF8D55B500BA930EED4AFEC8F5280CEAD706000000000000000000000000"),
        uint256_ops::get_uint256("FE2B8A089B3DCDE898F619E72D13430DFECEBC3E000000000000000000000000"),
        uint256_ops::get_uint256("7B2E4308882F42F434D4B8BED2A9AD7BB533A429000000000000000000000000"),
        uint256_ops::get_uint256("160001BD6E0A616B783A3AAE0F9585C0C41DFDD0000000000000000000000000"),
        uint256_ops::get_uint256("B64BC7723EA8EEB95ECE351DD17BFB54EB15BB90000000000000000000000000"),
        uint256_ops::get_uint256("7AF89C666F90977DAF86E8ED895C41B24ACED94B000000000000000000000000"),
        uint256_ops::get_uint256("49E4F332552805B0FF1A709D2B3CEE7F253DF5DC000000000000000000000000"),
        uint256_ops::get_uint256("E7CA24C8B71B90101674827F55019CDBBA483AF4000000000000000000000000"),
        uint256_ops::get_uint256("FF6037B93FFD47F63BA0CF29A75FA7DBF6BBAB52000000000000000000000000"),
        uint256_ops::get_uint256("2B1840EE97DDFAADDAEB132AF7A154B51FD8B08C000000000000000000000000"),
        uint256_ops::get_uint256("89994C4A71FD5B05BA2F8BFA42B977E6C9798E1F000000000000000000000000"),
        uint256_ops::get_uint256("2A07D4FE6045DB8E1B092CFF05BDE19652779D37000000000000000000000000"),
        uint256_ops::get_uint256("95424BDA14AB2CB02B57BB5EA1FEC953D0E8F986000000000000000000000000"),
        uint256_ops::get_uint256("5F96FC1F1146EB86233E1BC84E38CA4983DE045A000000000000000000000000")
    ];
    let mut friQueue = vec![
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001012"),
        uint256_ops::get_uint256("011462121fb1387a235c051031d539ce8e750f46a279b15c06b2344d40eea3be"),
        uint256_ops::get_uint256("04a4fe0d0fc4cc7019be9cfe12dbe6e34a4a24a5277ce225cd0fca2e433e3390"),
        uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010cb"),
        uint256_ops::get_uint256("042b8c16470e756a27007c1e6e255494a908f3f27e13773c27a14c864cfad334"),
        uint256_ops::get_uint256("016ee54bcb725324d9a77a7403c50e1778c52e39418f68360388ffdb8a2e7b40"),
        uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000010d5"),
        uint256_ops::get_uint256("07bf54d5d95260713415224d1ac3f43806112288793cc56f069a82dc5176f270"),
        uint256_ops::get_uint256("01d143bab1873a7d4f202199f91650095e9868722bc42a014a2a0a0cbac2a11e"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001367"),
        uint256_ops::get_uint256("00db11e36ce9d5696d7ad06c04b69bede5e7afe900d0457e57514e770d979162"),
        uint256_ops::get_uint256("0151dd6716be703689175ccb3382aee2e0e680238b8c37991fa30fc4e4d3817f"),
        uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000014e3"),
        uint256_ops::get_uint256("05d2ce3e77d3f0fbb700747c4449e2673e8f6f76de40ea692e2b905328aad6a6"),
        uint256_ops::get_uint256("0194ed93a6f7bfb2a717179a572db8206258fff8a53bf7dc0a79bb20cfddac29"),
        uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000015eb"),
        uint256_ops::get_uint256("027d00e17f162358fb96f69bb37d4d3551fd3217091463a64fad40a92c0ce228"),
        uint256_ops::get_uint256("07d314a158f16ae7f9f8ef10337bce1fb170f19d3c4f233711e2954276062b88"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001654"),
        uint256_ops::get_uint256("00583e71b55acd93a70ab68bcca029f2dd4169e4ffc5fca2d1f029be0935c59b"),
        uint256_ops::get_uint256("028636f271a6624cccabbf60a345b401bb6a6348b35446d22182d71fb02f94c2"),
        uint256_ops::get_uint256("00000000000000000000000000000000000000000000000000000000000017e0"),
        uint256_ops::get_uint256("05c3d803111c8d29e499ee8fb6022379a3ea6a92964013d5bc51701a35f4ce0c"),
        uint256_ops::get_uint256("0048902737f37b5f39d1ee9f1ed776883acdcac3a48cc526b83da80641c1dddb"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ad0"),
        uint256_ops::get_uint256("02f843dd049ad582234eac82a29e453e37b41f58d8ea4f6f4282f3dacd975f3f"),
        uint256_ops::get_uint256("04fc4b40f37d705cd1c9415d6d5a2833a07e6199c682491b586c5ea03f045500"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001ece"),
        uint256_ops::get_uint256("066481d9ad6dbac0f9d98f65c06b82d7be056e0f01c1be76a6dbe03f55b03e7f"),
        uint256_ops::get_uint256("04f7e3be67a9c6d111c6663370aa7a11d521411b24ad2492dab13f745aaa6eab"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001eda"),
        uint256_ops::get_uint256("0294c7459dfb0e0a3ae9ab2e17b0df5d549673e15a994c9bde177f3389a32680"),
        uint256_ops::get_uint256("02733978296e6604476ed88825042d9c2fe43799c1381efe023b00cc90cf9b09"),
        uint256_ops::get_uint256("0000000000000000000000000000000000000000000000000000000000001f3d"),
        uint256_ops::get_uint256("01a545ce07207992a0188ad8762bf0a032cc779974fc5d3d3d868a0be0051718"),
        uint256_ops::get_uint256("051faff8712b004841ebaa11f23191c9e5a1157489f01c5305ce18606948cec9"),
        uint256_ops::get_uint256("0")
    ];

    let evaluationPoint = uint256_ops::get_uint256("1E2ACDCED5CE1C2C6CD77A8CA31515B0A75FA8C7EFDC38C311FF00D23BF4E0F");
    let expectedRoot = uint256_ops::get_uint256("7FF714006C0A255A7B0CBF77E138196383ACAC52000000000000000000000000");

    let friStepSize = 2;

    fri::verifyFRI(proof, &mut friQueue, evaluationPoint, friStepSize, expectedRoot);
}
