#include "keymaps.au3"
#include <File.au3>
#include <GUIConstantsEx.au3>
; Modify this in case of lag or poor framerate to a higher number
Opt("SendKeyDelay", 200)
Opt("SendKeyDownDelay", 30)
Opt("MouseClickDownDelay", 100)
Opt("GUIOnEventMode", 1) ; Change to OnEvent mode
Opt('GUICoordMode', 0)


Global $kRecipeDir = @ScriptDir & "\recipes\"

Func WaitSeconds($amt)
    Sleep($amt * 1000)
 EndFunc

Func SendKey($key)
   ControlSend($hWnd, "", "", $key, 0)
EndFunc

Func ListRecipes(ByRef $recipe)
    $files = _FileListToArray($kRecipeDir, "*.txt");, $FLTA_FILES)
	if @error Then
	   MsgBox($IDOK, "Error!", "Error: " & @error & @CR)
	   Exit
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

; Takes a line from a macro and splits it into the action and the time needed
; to wait before the next gcd.
Func LineToactions(ByRef $line, ByRef $action, ByRef $wait)
   local $split = StringRegExp($line, "/ac [" & '"' & "]*([a-zA-Z': ]+)[" & '"' & "]* <wait.(\d)>", $STR_REGEXPARRAYMATCH)
   if @error Then
	  MsgBox($IDOK, "Error!", "Couldn't split: '" & $line & "'" & @CR)
	  Exit
   EndIf
   $action = $split[0]
   $wait = $split[1]
EndFunc

Func RunRecipe(Const $recipe, Const $count)
   $file_contents = FileRead($kRecipeDir & $recipe)
   If @error Then
	  MsgBox("$Error reading recipe file: " & @error & @CR)
	  Exit
   EndIf

   ; Get one action per line
   $actions = StringSplit($file_contents, @CR)

   ; First verify we have a keybind for the actions in the macro
   For $i = 1 To $actions[0] - 1
	  local $action, $wait
	  LineToActions($actions[$i], $action, $wait)
	  ConsoleWrite("action: '" & $action & "', wait:  '" & $wait & "'" & @CR)
	  if $actionMap.item($action) == "" Then
		 MsgBox($IDOK, "Error!", "Missing keymap for '" & $action & "'! Aborting..." & @CR)
		 Exit
	  EndIf
   Next

   ; If we made it this far we should be good to run the macro
   For $i = 1 To $actions[0] - 1
	  local $action, $wait
	  LineToActions($actions[$i], $action, $wait)
	  ConsoleWrite("executing '" & $action & "'" & @CR)
	  SendKey($actionMap.item($action))
	  WaitSeconds($wait)
   Next
EndFunc

; Brings up the crafting log, types in the recipe, and then
; moves the cursor to the Craft button

Func BringUpRecipeWindow(Const $recipe)
    ; Clear the crafting window to reset the state
	SendKey("{ESC}")
    SendKey($actionMap("Crafting Log"))
    WaitSeconds(1)
    SendKey($actionMap("Cycle Forward"))
    SendKey($actionMap("Cycle Forward"))
    SendKey($actionMap("Confirm"))
    SendKey($recipe & "{ENTER}")
    WaitSeconds(1)
 EndFunc

Global $UICount
Global $UICombo
Global $MacroList
Func CloseGUI()
   GUIDelete()
   Exit
EndFunc

Func CreateUI($recipes)

   GUICreate("Crafting Lalafell", 350, 200, -1, -1)
   $UICombo = GUICtrlCreateCombo("weeklies", 30, 30)
   $MacroList = _FileListToArray($kRecipeDir, "*.txt")
   $cData = ""
   For $i = 1 To Ubound($MacroList)-1
	  $cData &= "|" & $MacroList[$i]
   Next
   GUICtrlSetData($UICombo, $cData, $MacroList[1])
   GUICtrlCreateLabel("Count: ", 0, 30)
   $UICount = GUICtrlCreateInput("1", 50, 0)
   $UIButton = GUICtrlCreateButton("Craft", 50, -1)
   GUICtrlSetOnEvent($UIButton, "CraftCallback")
   GUISetOnEvent($GUI_EVENT_CLOSE, "CloseGUI")
   GUICtrlCreateLabel("Before hitting 'Craft' remember to do the following:" & @CR & _
					  "1. Search for the recipe and set the NQ/HQ mats" & @CR & _
					  "2. Ensure the recipe is selected and not other UI elements by " & @CR & _
					  "    clicking on the recipe name in search", -100, 30, 300, 100)
   GUISetState(@SW_SHOW)
EndFunc

Func CraftCallback()
   GUISetState(@SW_HIDE)
   $hWnd = WinWait("FINAL FANTASY XIV", "", 10)
   WinActivate($hWnd)

   for $i = 1 to Number(GUICtrlRead($UICount))
	  SendKey($actionMap("Confirm"))
	  SendKey($actionMap("Confirm"))
	  SendKey($actionMap("Confirm"))
	  WaitSeconds(2) ; Delay waiting for the craft progress window to pop up
	  RunRecipe($MacroList[$UICombo + 1], 1)
	  WaitSeconds(2) ; wait for the item completion to finish
   Next
   SendKey("{ESC}")

   Exit
EndFunc
local $recipes
ListRecipes($recipes)
CreateUI($recipes)

; Idle loop while waiting for button press
While 1
   Sleep(10)
Wend
