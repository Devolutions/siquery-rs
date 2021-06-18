Function New-SiqOSSpecificFunctions {
	[CmdletBinding()]
	param ()

	Begin {
		$GenericFunctionTemplate = "function Get-Siq{Table} {`n`t[CmdletBinding()]`n`tparam()`n`n`tGet-SiqTableJson '{TableName}'`n}"
		$Tables = Get-SiqTableName
		$Tables2 = $tables | ForEach-Object {
			[PSCustomOBject]@{
				TableName         = $_
				TableNoUnderScore = (ConvertTo-PascalCase $_).Replace('_', '')
			}
		}
		# $TableSchemas = Get-SiqTableSchema
		$PublicFolder = Get-Item "$PSScriptRoot\..\Public"
		$PublicFunctions = Get-ChildItem "$PublicFolder\*.ps1"
	}

	Process {
		$ExistingTables = Foreach ($Function in $PublicFunctions) {
			if ($Function.BaseName -imatch 'Get-Siq(?<TableName>\w*)') { $Matches['TableName'] }
		}
		Foreach ($Table in $Tables2) {
			if ($ExistingTables -inotcontains $Table.'TableNoUnderScore') {
				$TableFunction = $GenericFunctionTemplate.Replace('{Table}', $Table.'TableNoUnderScore')
				$TableFunction = $TableFunction.Replace('{TableName}', $Table.'TableName')

				$FunctionName = "Get-Siq$($Table.'TableNoUnderScore')"
				New-Item -Path "$PublicFolder\$FunctionName.ps1" -ItemType File -Value $TableFunction | Out-Null
				. ([ScriptBlock]::Create($TableFunction))
				Export-ModuleMember -Function $FunctionName -Alias '*'
			}
		}
	}

	End {}
}
