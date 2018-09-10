#include "keymap.au3"
#include "recipes.au3"
Global $actions;

If @error Then
    MsgBox(0, '', 'Error creating the dictionary object')
	Exit()
EndIf




Global $recipe = $r_D35_L50_58
Global $num = 7

Func do_step($step)
	Local $step_elems = StringSplit($step, ",")
	Local $key = $step_elems[1]
	Local $short_key = $step_elems[2]
    ;WinWaitActive("FINAL FANTASY XIV: A Realm Rebocrn")
    ;Send($key)
   Local $hWnd = WinWait("FINAL FANTASY XIV", "", 10)
   ControlSend($hWnd, "", "", $key)
   ;MsgBox(0, '', 'key ' & $key)
	If $short_key = "true" Then
		return 1550
	EndIf

	return 2600
EndFunc

Func delay($amt)
    $x = ($amt * 1200) / 1000
    Sleep($x)
EndFunc

Func click($x, $y)
        WinWaitActive("FINAL FANTASY XIV")
        MouseMove($x, $y)
        Sleep(300)
        MouseDown("left")
        Sleep(100)
        MouseUp("left")
        Sleep(100)
EndFunc

WinActivate("FINAL FANTASY XIV")
Opt("SendKeyDelay", 200)
Opt("SendKeyDownDelay", 30)
Opt("MouseClickDownDelay", 100)

$t = 0
For $i = 1 To $num

   delay($t)
	$t = 3500
	  WinWaitActive("FINAL FANTASY XIV")
   ;MouseClick("left",904,740)
	;MouseClick("left",904,740)
	$x = 2243
	$y = 1229
	click($x, $y)
	click($x, $y)

	$s = 1800
	For $step In $recipe
		;MsgBox(0, '', 'delay ' & $s)
		delay($s)
		$s = do_step($actionMap($step))
	Next
Next
;MsgBox(0, '', 'success')
