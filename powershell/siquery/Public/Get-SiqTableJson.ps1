function Get-SiqTableJson {
	[CmdletBinding()]
	param(
		[Parameter(Mandatory)]
		[string[]]$Name
	)

	Begin { $siq_exe = Find-SiqExecutable }
	Process {
		$TableNames = $Name | Sort-Object -Unique
		$Output = Foreach ($Table in $TableNames) {
			& "$siq_exe" '-a' $Table '--json' | ConvertFrom-Json | Select-Object -ExpandProperty $Table
		}
	}
	End { return $Output }
}
