Import-Module "$PSScriptRoot/../siquery"

Describe 'logon_sessions table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$logon_sessions = $allSchemas | Where-Object { $_.BaseName -eq 'logon_sessions' }
				$logon_sessions | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$logon_sessions = $allSchemas | Where-Object { $_.BaseName -eq 'logon_sessions' }
				$schema = Get-Content $logon_sessions.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$logon_sessions = Get-Siq 'logon_sessions' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'logon_sessions' }).FullName -Raw

				$json = $logon_sessions | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
