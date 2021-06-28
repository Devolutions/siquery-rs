Import-Module "$PSScriptRoot/../siquery"

Describe 'process_open_sockets table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$process_open_sockets = $allSchemas | Where-Object { $_.BaseName -eq 'process_open_sockets' }
				$process_open_sockets | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$process_open_sockets = $allSchemas | Where-Object { $_.BaseName -eq 'process_open_sockets' }
				$schema = Get-Content $process_open_sockets.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$process_open_sockets = Get-Siq 'process_open_sockets' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'process_open_sockets' }).FullName -Raw

				$json = $process_open_sockets | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
