function Get-Siq {
	[CmdletBinding()]
	param(
		[Parameter( Mandatory, Position = 0, ValueFromPipeline )]
		[Alias('Name', 'Table')]
		[string]$TableName
	)

	Begin {
		$ErrorActionPreference = 'Stop'
		$siq_exe = Find-SiqExecutable
	}

	Process { $Output = & "$siq_exe" '-a' $TableName '--json' | ConvertFrom-Json }

	End { return $Output.$TableName }
}
