$Folders = ls $PSScriptRoot/src -Directory

foreach ($i in 0..($Folders.Length-1)) {
  $Files = ls $Folders[$i].FullName

  foreach ($File in $Files) {
    $Name = [int]$File.BaseName
    $Num = ($i + 1) * 1000
    $NextNum = ($i + 2) * 1000

    if (!($Name -ge $Num -and $Name -lt $NextNum)) {
      echo "$Num $Name.rs"
    }
  }
}

echo "Done"
read-host