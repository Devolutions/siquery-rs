function Find-SiqExecutable {
	[CmdletBinding()]
	param()

	if (Test-Path Env:\SIQUERY) { return $Env:SIQUERY }

	$command = Get-Command -Name 'siquery' -CommandType 'Application' -ErrorAction SilentlyContinue
	if (-Not $command) { throw 'could not find siquery executable' }

	Return $command.Source
}
