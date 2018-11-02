extern crate exoquant;

use Color;

pub fn get_vga_colors() -> [Color; 256] {
    let mut vga_colors: [Color; 256] = [Color::new(0, 0, 0, 0); 256];

    /* First row */
    vga_colors[0] = Color::new(0, 0, 0, 1);
    vga_colors[1] = Color::new(0, 0, 170, 0);
    vga_colors[2] = Color::new(0, 170, 0, 0);
    vga_colors[3] = Color::new(0, 170, 170, 0);
    vga_colors[4] = Color::new(170, 0, 0, 0);
    vga_colors[5] = Color::new(170, 0, 170, 0);
    vga_colors[6] = Color::new(170, 85, 0, 0);
    vga_colors[7] = Color::new(170, 170, 170, 0);
    vga_colors[8] = Color::new(85, 85, 85, 0);
    vga_colors[9] = Color::new(85, 85, 255, 0);
    vga_colors[10] = Color::new(85, 255, 85, 0);
    vga_colors[11] = Color::new(85, 255, 255, 0);
    vga_colors[12] = Color::new(255, 85, 85, 0);
    vga_colors[13] = Color::new(255, 85, 255, 0);
    vga_colors[14] = Color::new(255, 255, 85, 0);
    vga_colors[15] = Color::new(255, 255, 255, 0);

    /* Second row */
    vga_colors[16] = Color::new(0, 0, 0, 0);
    vga_colors[17] = Color::new(16, 16, 16, 0);
    vga_colors[18] = Color::new(32, 32, 32, 0);
    vga_colors[19] = Color::new(53, 53, 53, 0);
    vga_colors[20] = Color::new(69, 69, 69, 0);
    vga_colors[21] = Color::new(85, 85, 85, 0);
    vga_colors[22] = Color::new(101, 101, 101, 0);
    vga_colors[23] = Color::new(117, 117, 117, 0);
    vga_colors[24] = Color::new(138, 138, 138, 0);
    vga_colors[25] = Color::new(154, 154, 154, 0);
    vga_colors[26] = Color::new(170, 170, 170, 0);
    vga_colors[27] = Color::new(186, 186, 186, 0);
    vga_colors[28] = Color::new(202, 202, 202, 0);
    vga_colors[29] = Color::new(223, 223, 223, 0);
    vga_colors[30] = Color::new(239, 239, 239, 0);
    vga_colors[31] = Color::new(255, 255, 255, 0);

    /* Third row */
    vga_colors[32] = Color::new(0, 0, 255, 0);
    vga_colors[33] = Color::new(65, 0, 255, 0);
    vga_colors[34] = Color::new(130, 0, 255, 0);
    vga_colors[35] = Color::new(190, 0, 255, 0);
    vga_colors[36] = Color::new(255, 0, 255, 0);
    vga_colors[37] = Color::new(255, 0, 190, 0);
    vga_colors[38] = Color::new(255, 0, 130, 0);
    vga_colors[39] = Color::new(255, 0, 65, 0);
    vga_colors[40] = Color::new(255, 0, 0, 0);
    vga_colors[41] = Color::new(255, 65, 0, 0);
    vga_colors[42] = Color::new(255, 130, 0, 0);
    vga_colors[43] = Color::new(255, 190, 0, 0);
    vga_colors[44] = Color::new(255, 255, 0, 0);
    vga_colors[45] = Color::new(190, 255, 0, 0);
    vga_colors[45] = Color::new(130, 255, 0, 0);
    vga_colors[47] = Color::new(65, 255, 0, 0);

    /* Fourth row */
    vga_colors[48] = Color::new(0, 255, 0, 0);
    vga_colors[49] = Color::new(0, 255, 65, 0);
    vga_colors[50] = Color::new(0, 255, 130, 0);
    vga_colors[51] = Color::new(0, 255, 190, 0);
    vga_colors[52] = Color::new(0, 255, 255, 0);
    vga_colors[53] = Color::new(0, 190, 255, 0);
    vga_colors[54] = Color::new(0, 130, 255, 0);
    vga_colors[55] = Color::new(0, 65, 255, 0);
    vga_colors[56] = Color::new(130, 130, 255, 0);
    vga_colors[57] = Color::new(158, 130, 255, 0);
    vga_colors[58] = Color::new(190, 130, 255, 0);
    vga_colors[59] = Color::new(223, 130, 255, 0);
    vga_colors[60] = Color::new(255, 130, 255, 0);
    vga_colors[61] = Color::new(255, 130, 223, 0);
    vga_colors[62] = Color::new(255, 130, 190, 0);
    vga_colors[63] = Color::new(255, 130, 158, 0);

    /* Fifth row */
    vga_colors[64] = Color::new(255, 130, 130, 0);
    vga_colors[65] = Color::new(255, 158, 130, 0);
    vga_colors[66] = Color::new(255, 190, 130, 0);
    vga_colors[67] = Color::new(255, 223, 130, 0);
    vga_colors[68] = Color::new(255, 255, 130, 0);
    vga_colors[69] = Color::new(223, 255, 130, 0);
    vga_colors[70] = Color::new(190, 255, 130, 0);
    vga_colors[71] = Color::new(158, 255, 130, 0);
    vga_colors[72] = Color::new(130, 255, 130, 0);
    vga_colors[73] = Color::new(130, 255, 158, 0);
    vga_colors[74] = Color::new(130, 255, 190, 0);
    vga_colors[75] = Color::new(130, 255, 223, 0);
    vga_colors[76] = Color::new(130, 255, 255, 0);
    vga_colors[77] = Color::new(130, 223, 255, 0);
    vga_colors[78] = Color::new(130, 190, 255, 0);
    vga_colors[79] = Color::new(130, 158, 255, 0);

    /* Sixth row */
    vga_colors[80] = Color::new(186, 186, 255, 0);
    vga_colors[81] = Color::new(202, 186, 255, 0);
    vga_colors[82] = Color::new(223, 186, 255, 0);
    vga_colors[83] = Color::new(239, 186, 255, 0);
    vga_colors[84] = Color::new(255, 186, 255, 0);
    vga_colors[85] = Color::new(255, 186, 239, 0);
    vga_colors[86] = Color::new(255, 186, 223, 0);
    vga_colors[87] = Color::new(255, 186, 202, 0);
    vga_colors[88] = Color::new(255, 186, 186, 0);
    vga_colors[89] = Color::new(255, 202, 186, 0);
    vga_colors[90] = Color::new(255, 223, 186, 0);
    vga_colors[91] = Color::new(255, 239, 186, 0);
    vga_colors[92] = Color::new(255, 255, 186, 0);
    vga_colors[93] = Color::new(239, 255, 186, 0);
    vga_colors[94] = Color::new(223, 255, 186, 0);
    vga_colors[95] = Color::new(202, 255, 186, 0);

    /* Seventh row */
    vga_colors[96] = Color::new(186, 255, 186, 0);
    vga_colors[97] = Color::new(186, 255, 202, 0);
    vga_colors[98] = Color::new(186, 255, 223, 0);
    vga_colors[99] = Color::new(186, 255, 239, 0);
    vga_colors[100] = Color::new(186, 255, 255, 0);
    vga_colors[101] = Color::new(186, 239, 255, 0);
    vga_colors[102] = Color::new(186, 223, 255, 0);
    vga_colors[103] = Color::new(186, 202, 255, 0);
    vga_colors[104] = Color::new(0, 0, 113, 0);
    vga_colors[105] = Color::new(28, 0, 113, 0);
    vga_colors[106] = Color::new(57, 0, 113, 0);
    vga_colors[107] = Color::new(85, 0, 113, 0);
    vga_colors[108] = Color::new(113, 0, 113, 0);
    vga_colors[109] = Color::new(113, 0, 85, 0);
    vga_colors[110] = Color::new(113, 0, 57, 0);
    vga_colors[111] = Color::new(113, 0, 28, 0);

    /* Eighth row */
    vga_colors[112] = Color::new(113, 0, 0, 0);
    vga_colors[113] = Color::new(113, 28, 0, 0);
    vga_colors[114] = Color::new(113, 57, 0, 0);
    vga_colors[115] = Color::new(113, 85, 0, 0);
    vga_colors[116] = Color::new(113, 113, 0, 0);
    vga_colors[117] = Color::new(85, 113, 0, 0);
    vga_colors[118] = Color::new(57, 113, 0, 0);
    vga_colors[119] = Color::new(28, 113, 0, 0);
    vga_colors[120] = Color::new(0, 113, 0, 0);
    vga_colors[121] = Color::new(0, 113, 28, 0);
    vga_colors[122] = Color::new(0, 113, 57, 0);
    vga_colors[123] = Color::new(0, 113, 85, 0);
    vga_colors[124] = Color::new(0, 113, 113, 0);
    vga_colors[125] = Color::new(0, 85, 113, 0);
    vga_colors[126] = Color::new(0, 57, 113, 0);
    vga_colors[127] = Color::new(0, 28, 113, 0);

    /* Ninth row */
    vga_colors[128] = Color::new(57, 57, 113, 0);
    vga_colors[129] = Color::new(69, 57, 113, 0);
    vga_colors[130] = Color::new(85, 57, 113, 0);
    vga_colors[131] = Color::new(97, 57, 113, 0);
    vga_colors[132] = Color::new(113, 57, 113, 0);
    vga_colors[133] = Color::new(113, 57, 97, 0);
    vga_colors[134] = Color::new(113, 57, 85, 0);
    vga_colors[135] = Color::new(113, 57, 69, 0);
    vga_colors[136] = Color::new(113, 57, 57, 0);
    vga_colors[137] = Color::new(113, 69, 57, 0);
    vga_colors[138] = Color::new(113, 85, 57, 0);
    vga_colors[139] = Color::new(113, 97, 57, 0);
    vga_colors[140] = Color::new(113, 113, 57, 0);
    vga_colors[141] = Color::new(97, 113, 57, 0);
    vga_colors[142] = Color::new(85, 113, 57, 0);
    vga_colors[143] = Color::new(69, 113, 57, 0);

    /* Tenth row */
    vga_colors[144] = Color::new(57, 113, 57, 0);
    vga_colors[145] = Color::new(57, 113, 69, 0);
    vga_colors[146] = Color::new(57, 113, 85, 0);
    vga_colors[147] = Color::new(57, 113, 97, 0);
    vga_colors[148] = Color::new(57, 113, 113, 0);
    vga_colors[149] = Color::new(57, 97, 113, 0);
    vga_colors[150] = Color::new(57, 85, 113, 0);
    vga_colors[151] = Color::new(57, 69, 113, 0);
    vga_colors[152] = Color::new(81, 81, 113, 0);
    vga_colors[153] = Color::new(89, 81, 113, 0);
    vga_colors[154] = Color::new(97, 81, 113, 0);
    vga_colors[155] = Color::new(105, 81, 113, 0);
    vga_colors[156] = Color::new(113, 81, 113, 0);
    vga_colors[157] = Color::new(113, 81, 105, 0);
    vga_colors[158] = Color::new(113, 81, 97, 0);
    vga_colors[159] = Color::new(113, 81, 89, 0);

    /* Eleventh row */
    vga_colors[160] = Color::new(113, 81, 81, 0);
    vga_colors[161] = Color::new(113, 89, 81, 0);
    vga_colors[162] = Color::new(113, 97, 81, 0);
    vga_colors[163] = Color::new(113, 105, 81, 0);
    vga_colors[164] = Color::new(113, 113, 81, 0);
    vga_colors[165] = Color::new(105, 113, 81, 0);
    vga_colors[166] = Color::new(97, 113, 81, 0);
    vga_colors[167] = Color::new(89, 113, 81, 0);
    vga_colors[168] = Color::new(81, 113, 81, 0);
    vga_colors[169] = Color::new(81, 113, 89, 0);
    vga_colors[170] = Color::new(81, 113, 97, 0);
    vga_colors[171] = Color::new(81, 113, 105, 0);
    vga_colors[172] = Color::new(81, 113, 113, 0);
    vga_colors[173] = Color::new(81, 105, 113, 0);
    vga_colors[174] = Color::new(81, 97, 113, 0);
    vga_colors[175] = Color::new(81, 89, 113, 0);

    /* Twelfth row */
    vga_colors[176] = Color::new(0, 0, 65, 0);
    vga_colors[177] = Color::new(16, 0, 65, 0);
    vga_colors[178] = Color::new(32, 0, 65, 0);
    vga_colors[179] = Color::new(49, 0, 65, 0);
    vga_colors[180] = Color::new(65, 0, 65, 0);
    vga_colors[181] = Color::new(65, 0, 49, 0);
    vga_colors[182] = Color::new(65, 0, 32, 0);
    vga_colors[183] = Color::new(65, 0, 16, 0);
    vga_colors[184] = Color::new(65, 0, 0, 0);
    vga_colors[185] = Color::new(65, 16, 0, 0);
    vga_colors[186] = Color::new(65, 32, 0, 0);
    vga_colors[187] = Color::new(65, 49, 0, 0);
    vga_colors[188] = Color::new(65, 65, 0, 0);
    vga_colors[189] = Color::new(49, 65, 0, 0);
    vga_colors[190] = Color::new(32, 65, 0, 0);
    vga_colors[191] = Color::new(16, 65, 0, 0);

    /* Thirteenth row */
    vga_colors[192] = Color::new(0, 65, 0, 0);
    vga_colors[193] = Color::new(0, 65, 16, 0);
    vga_colors[194] = Color::new(0, 65, 32, 0);
    vga_colors[195] = Color::new(0, 65, 49, 0);
    vga_colors[196] = Color::new(0, 65, 65, 0);
    vga_colors[197] = Color::new(0, 49, 65, 0);
    vga_colors[198] = Color::new(0, 32, 65, 0);
    vga_colors[199] = Color::new(0, 16, 65, 0);
    vga_colors[200] = Color::new(32, 32, 65, 0);
    vga_colors[201] = Color::new(40, 32, 65, 0);
    vga_colors[202] = Color::new(49, 32, 65, 0);
    vga_colors[203] = Color::new(57, 32, 65, 0);
    vga_colors[204] = Color::new(65, 32, 65, 0);
    vga_colors[205] = Color::new(65, 32, 57, 0);
    vga_colors[206] = Color::new(65, 32, 49, 0);
    vga_colors[207] = Color::new(65, 32, 40, 0);

    /* Fourteenth row */
    vga_colors[208] = Color::new(65, 32, 32, 0);
    vga_colors[209] = Color::new(65, 40, 32, 0);
    vga_colors[210] = Color::new(65, 49, 32, 0);
    vga_colors[211] = Color::new(65, 57, 32, 0);
    vga_colors[212] = Color::new(65, 65, 32, 0);
    vga_colors[213] = Color::new(57, 65, 32, 0);
    vga_colors[214] = Color::new(49, 65, 32, 0);
    vga_colors[215] = Color::new(40, 65, 32, 0);
    vga_colors[216] = Color::new(32, 65, 32, 0);
    vga_colors[217] = Color::new(32, 65, 40, 0);
    vga_colors[218] = Color::new(32, 65, 49, 0);
    vga_colors[219] = Color::new(32, 65, 57, 0);
    vga_colors[220] = Color::new(32, 65, 65, 0);
    vga_colors[221] = Color::new(32, 57, 65, 0);
    vga_colors[222] = Color::new(32, 49, 65, 0);
    vga_colors[223] = Color::new(32, 40, 65, 0);

    /* Fifteenth row */
    vga_colors[224] = Color::new(45, 45, 65, 0);
    vga_colors[225] = Color::new(49, 45, 65, 0);
    vga_colors[226] = Color::new(53, 45, 65, 0);
    vga_colors[227] = Color::new(61, 45, 65, 0);
    vga_colors[228] = Color::new(65, 45, 65, 0);
    vga_colors[229] = Color::new(65, 45, 61, 0);
    vga_colors[230] = Color::new(65, 45, 53, 0);
    vga_colors[231] = Color::new(65, 45, 49, 0);
    vga_colors[232] = Color::new(65, 45, 45, 0);
    vga_colors[233] = Color::new(65, 49, 45, 0);
    vga_colors[234] = Color::new(65, 53, 45, 0);
    vga_colors[235] = Color::new(65, 61, 45, 0);
    vga_colors[236] = Color::new(65, 65, 45, 0);
    vga_colors[237] = Color::new(61, 65, 45, 0);
    vga_colors[238] = Color::new(53, 65, 45, 0);
    vga_colors[239] = Color::new(49, 65, 45, 0);

    /* Sixteenth row */
    vga_colors[240] = Color::new(45, 65, 45, 0);
    vga_colors[241] = Color::new(45, 65, 49, 0);
    vga_colors[242] = Color::new(45, 65, 53, 0);
    vga_colors[243] = Color::new(45, 65, 61, 0);
    vga_colors[244] = Color::new(45, 65, 65, 0);
    vga_colors[245] = Color::new(45, 61, 65, 0);
    vga_colors[246] = Color::new(45, 53, 65, 0);
    vga_colors[247] = Color::new(45, 49, 65, 0);
    vga_colors[248] = Color::new(0, 0, 0, 0);
    vga_colors[249] = Color::new(0, 0, 0, 0);
    vga_colors[250] = Color::new(0, 0, 0, 0);
    vga_colors[251] = Color::new(0, 0, 0, 0);
    vga_colors[252] = Color::new(0, 0, 0, 0);
    vga_colors[253] = Color::new(0, 0, 0, 0);
    vga_colors[254] = Color::new(0, 0, 0, 0);
    vga_colors[255] = Color::new(0, 0, 0, 0);

    vga_colors
}
