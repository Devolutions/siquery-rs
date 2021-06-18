function Get-Siq {
	[CmdletBinding()]
	param(
		[Parameter(Mandatory,Position=0)]
		[string] $Name
	)

	Begin {
		$siq_exe = Find-SiqExecutable
	} Process {
		& "$siq_exe" '-a' $Name '--json' | ConvertFrom-Json | Select-Object -ExpandProperty $Name
	} End {
		return $Output
	}
}
