// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

	// スクリーン用のRAMメモリ数
	@8192 // 16bitワードが、32個(横512ピクセル) * 256行(縦256ピクセル) = 8,192
	D=A
	@ram_width_for_pixels
	M=D

(LOOP)
	@counter
	M=0

	@KBD // キーボード用 入出力ポインタ
	D=M
	@input
	M=D

	// 何も入力が無ければスクリーンを白で埋める
	@input
	D=M
	@FILL_WHITE
	D;JEQ

	// 何か入力があればスクリーンを黒で埋める
	@input
	D=M
	@FILL_BLACK
	D;JGT

(FILL_WHITE)
	// ループが目的の回数に達したらメインループに戻る
	@ram_width_for_pixels
	D=M
	@counter
	D=D-M
	@LOOP
	D;JLE

	// 白で埋める
	@SCREEN
	D=A
	@counter
	A=D+M
	M=0

	@counter
	M=M+1
	@FILL_WHITE
	0;JMP

(FILL_BLACK)
	// ループが目的の回数に達したらメインループに戻る
	@ram_width_for_pixels
	D=M
	@counter
	D=D-M
	@LOOP
	D;JLE

	// 黒で埋める
	@SCREEN
	D=A
	@counter
	A=D+M
	M=-1

	@counter
	M=M+1
	@FILL_BLACK
	0;JMP
