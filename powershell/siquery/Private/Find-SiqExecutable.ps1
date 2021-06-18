function Find-SiqExecutable {
	[CmdletBinding()]
	param()

	if (Test-Path Env:\SIQUERY) {
		return $Env:SIQUERY
	}

	$command = Get-Command -Name 'siquery' -CommandType 'Application' -ErrorAction SilentlyContinue

	if ($command) {
		return $command.Source
	}

	$ExecutableName = "siquery"
	if ($IsWindows) {
		$ExecutableName += ".exe"
	}

	$ExecutablePath = Join-Path $(Split-Path $PSScriptRoot -Parent) "bin" $ExecutableName

	if (Test-Path $ExecutablePath) {
		return $ExecutablePath
	}

	throw "could not find siquery executable!"
}
