(START)
(DEFINE_CONSTANTS)
// define first number
  @R0
  D=M
  @first_number
  M=D

// define second number
  @R1
  D=M
  @second_number
  M=D

(DEFINE_VARIABLES)
  @how_many_times_added
  M=0

  @accumulator
  M=0

(START)
  (CHECK_IF_ADDITION_IS_DONE)
  @second_number
  D=M
  @how_many_times_added
  D=D-M
  @SAVE_TO_REGISTER
  D; JEQ

  (ADD_TO_ACCUMULATOR)
  @first_number
  D=M
  @accumulator
  M=M+D

  (INCREMENT_COUNTER)
  @how_many_times_added
  M=M+1

  @CHECK_IF_ADDITION_IS_DONE
  0; JMP

  (SAVE_TO_REGISTER)
  @accumulator
  D=M
  @R2
  M=D

(END)
  @END
  0; JMP
