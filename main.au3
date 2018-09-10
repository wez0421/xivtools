#include "keymaps.au3"
#include "recipes.au3"

Global $recipe = $r_D35_L50_58
Global $num = 7

Local $kRecipeDir = @ScriptDir & "\recipes\"
Func ListRecipes(ByRef $recipe)
    $files = _FileListToArray($kRecipeDir, "*.txt");, $FLTA_FILES)
	if @error Then
	   ConsoleWrite("Error: " & @error & @CR)
	Else
	  For $i = 1 To $files[0]
		 $file = $kRecipeDir & $files[$i]
		 $data = FileRead($file)
		 ;ConsoleWrite("Read in from " & $file & ":" & @CR & $data)
		 $cmds = StringSplit($data, @CR)
		 for $i = 1 to $cmds[0]
			$cmd = $cmds[$i]
			_ArrayInsert($recipe, StringRegExp($cmd, '/ac "(.*)" <wait.(\d)>', $STR_REGEXPARRAYMATCH))
		 next
	  Next
   EndIf
EndFunc

Func RunRecipe(Const $recipe, Const $count)
   $file_contents = FileRead($kRecipeDir & $recipe)
   If Not @error Then
	  $actions = StringSplit($file_contents, @CR)
	  For $i = 1 To $actions[0] - 1
		 $split = StringRegExp($actions[$i], '/ac "(.*)" <wait.(\d)>', $STR_REGEXPARRAYMATCH)
		 ConsoleWrite("[0] = " & $split[0] & " [1] = " & $split[1] & @CR)
		 ConsoleWrite("We got: " & $actionMap($split[0]) & " and to wait for " & $split[1] & " seconds "& @CR)
	  Next
   Else
	  ConsoleWrite("Error: " & @error & @CR)
   EndIf
EndFunc


#CS
Func do_step($step)
	Local $step_elems = StringSplit($step, ",")
	Local $key = $step_elems[1]
	Local $short_key = $step_elems[2]
    ;WinWaitActive("FINAL FANTASY XIV: A Realm Rebocrn")
    ;Send($key)
   Local $hWnd = WinWait("FINAL FANTASY XIV", "", 10)
   ControlSend($hWnd, "", "", $key)
   ;MsgBox(0, '', 'key ' & $key)
	;If $short_key = "true" Then
	;	return 1550
	;EndIf

	return 2600
EndFunc
#CE


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

;WinActivate("FINAL FANTASY XIV")
Opt("SendKeyDelay", 100)
Opt("SendKeyDownDelay", 30)
Opt("MouseClickDownDelay", 100)

RunRecipe("recipes.txt", 1)
;Local $hWnd = WinWait("FINAL FANTASY XIV", "", 10)
;ControlSend($hWnd, "", "", '/ac "Collectable Synthesis"{ENTER}')
;$t = 0
;For $i = 1 To $num
;
;   delay($t)
;	$t = 3500
;	  WinWaitActive("FINAL FANTASY XIV")
;   ;MouseClick("left",904,740)
;	;MouseClick("left",904,740)
;	$x = 2243
;	$y = 1229
;	click($x, $y)
;	click($x, $y)
;
;	$s = 1800
;	For $step In $recipe
;		;MsgBox(0, '', 'delay ' & $s)
;		delay($s)
;		$s = do_step($actionMap($step))
;	Next
;Next
;MsgBox(0, '', 'success')
