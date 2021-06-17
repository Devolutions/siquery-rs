Function New-SiqOSSpecificFunctions {
	[CmdletBinding()]
	param ()

	Begin {
		$GenericFunctionTemplate = "function Get-Siq{Table} {`n`t[CmdletBinding()]`n`tparam()`n`n`tGet-SiqTableJson '{TableName}'`n}"
		$Tables = Get-SiqTableName
		Write-Host "Found $($Tables.Count) Tables. Originally had $AllOsCompatCount"
		$Tables2 = $tables | ForEach-Object { [PSCustomOBject]@{ TableName = $_ ; TableNoUnderScore = (ConvertTo-PascalCase $_.Replace('_', '')) } }
		Write-Host "Found $($Tables2.Count) Tables2."
		$PublicFolder = Get-Item "$PSScriptRoot\..\Public"
		$PublicFunctions = Get-ChildItem "$PublicFolder\*.ps1"
		Write-Host "Found $($PublicFunctions.Count) Public Functions."
	}

	Process {
		$ExistingTables = Foreach ($Function in $PublicFunctions) {
			if ($Function.BaseName -imatch 'Get-Siq(?<TableName>\w*)') { $Matches['TableName'] }
		}
		Write-Host "Found $($ExistingTables.Count) Existing Tables."
		Foreach ($Table in $Tables2) {
			if ($ExistingTables -inotcontains $Table.'TableNoUnderScore') {
				$TableFunction = $GenericFunctionTemplate.Replace('{Table}', $Table.'TableNoUnderScore')
				$TableFunction = $TableFunction.Replace('{TableName}', $Table.'TableName')

				$FunctionName = "Get-Siq$($Table.'TableNoUnderScore')"
				New-Item -Path "$PublicFolder\$FunctionName.ps1" -ItemType File -Value $TableFunction -Verbose
				. ([ScriptBlock]::Create($TableFunction))
				Export-ModuleMember -Function $FunctionName -Alias '*' -Verbose
			} else {
				Write-Host "Module already contains a function for $($Table.'TableNoUnderScore')"
			}
		}
	}

	End {}
}
