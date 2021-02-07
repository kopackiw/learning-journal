(START)
(DEFINE_CONSTANTS)
  // define screen_begins_at
  @SCREEN
  D=A
  @screen_begins_at
  M=D

  // define screen_ends_at
  @8192
  D=A
  @screen_begins_at
  D=D+M
  @screen_ends_at
  M=D

(DEFINE_VARIABLES)
  // define current screen position
  @screen_begins_at
  D=M
  @current_screen_position
  M=D

  // define color to fill
  @color_to_fill
  M=0

(CHECK_KEYBOARD_INPUT)
  @KBD
  D=M
  @SET_COLOR_TO_WHITE
  D; JEQ

  (SET_COLOR_TO_BLACK)
  @color_to_fill
  M=-1
  @WRITE_TO_SCREEN
  0; JMP

  (SET_COLOR_TO_WHITE)
  @color_to_fill
  M=0

(WRITE_TO_SCREEN)
  (CHECK_IF_BOARD_IS_FILLED)
  @screen_ends_at
  D=M
  @current_screen_position
  D=D-M
  @RESET_SCREEN_POSITION_COUNTER
  D; JEQ

  (PAINT)
  @color_to_fill
  D=M
  @current_screen_position
  A=M
  M=D
  @current_screen_position
  M=M+1
  @CHECK_IF_BOARD_IS_FILLED
  0; JMP

  (RESET_SCREEN_POSITION_COUNTER)
  @screen_begins_at
  D=M
  @current_screen_position
  M=D

  (START_LOOP_AGAIN)
  @CHECK_KEYBOARD_INPUT
  0; JMP
