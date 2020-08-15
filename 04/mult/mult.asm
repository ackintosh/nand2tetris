// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// 合計値を格納する sum を初期化
@sum
M=0
// ループ用のカウンタ i を初期化
@i
M=0

(LOOP)
	// ループが目的の回数に達したらENDにジャンプする
	@i
	D=M
	@R1
	D=D-M
	@END
	D;JEQ

	// 加算
	@R0
	D=M
	@sum
	M=D+M

	// カウンタをインクリメント
	@i
	M=M+1
	// ループの先頭に戻る
	@LOOP
	0;JMP

@END
D;JEQ
(END)
	// 結果sumをRAM(R2)に保存する
	@sum
	D=M // D=sum
	@R2
	M=D // R2=sum

	// 終了
	@END
	0;JMP
